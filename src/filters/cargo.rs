use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct CargoFilter;

impl CommandFilter for CargoFilter {
    fn name(&self) -> &str {
        "cargo"
    }

    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let subcommand = args.first().map(|s| s.as_str()).unwrap_or("");

        match subcommand {
            "test" => FilterResult::Filtered(filter_cargo_test(&output, exit_code)),
            "build" | "check" => FilterResult::Filtered(filter_cargo_build(&output, exit_code)),
            "clippy" => FilterResult::Filtered(filter_cargo_clippy(&output, exit_code)),
            "fmt" => FilterResult::Filtered(filter_cargo_fmt(&output, exit_code)),
            "run" => FilterResult::Filtered(generic::truncate_lines(&output, 50)),
            _ => FilterResult::Filtered(generic::truncate_lines(&output, 100)),
        }
    }
}

fn filter_cargo_test(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();

    let mut test_count = 0u32;
    let mut pass_count = 0u32;
    let mut fail_count = 0u32;
    let mut failures = Vec::new();
    let mut in_failure = false;
    let mut failure_lines = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("test ") || trimmed.starts_with("running ") {
            if trimmed.starts_with("test ") && !trimmed.starts_with("test result") {
                test_count += 1;
                if trimmed.contains("... ok") {
                    pass_count += 1;
                } else if trimmed.contains("... FAILED") {
                    fail_count += 1;
                }
            }
            continue;
        }

        if trimmed.starts_with("failures:") {
            in_failure = true;
            continue;
        }
        if in_failure {
            if trimmed.starts_with("----") && trimmed.ends_with("----") {
                failures.push(failure_lines.join("\n"));
                failure_lines = Vec::new();
                continue;
            }
            if trimmed.contains("panicked at")
                || trimmed.contains("assertion")
                || trimmed.contains("left:")
                || trimmed.contains("right:")
            {
                failure_lines.push(trimmed.to_string());
            }
            continue;
        }

        if trimmed.starts_with("test result:") {
            continue;
        }
    }

    if fail_count == 0 {
        return format!("PASSED: {}/{} tests", pass_count, test_count);
    }

    let mut result = vec![format!("FAILED: {}/{}\n", fail_count, test_count)];
    for (i, f) in failures.iter().enumerate() {
        if i < 10 {
            result.push(f.clone());
        }
    }
    if failures.len() > 10 {
        result.push(format!("... and {} more failures", failures.len() - 10));
    }
    result.join("\n")
}

fn filter_cargo_build(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if exit_code == Some(0) {
        let mut summary = Vec::new();
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("Compiling") || trimmed.starts_with("Finished") {
                summary.push(trimmed.to_string());
            }
        }
        if summary.is_empty() {
            return "build ok".to_string();
        }
        return summary.join("\n");
    }

    let mut errors = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("error")
            || trimmed.starts_with("Error")
            || trimmed.starts_with("  -->")
        {
            errors.push(trimmed.to_string());
        }
    }
    if errors.is_empty() {
        return generic::truncate_lines(output, 60);
    }
    errors.join("\n")
}

fn filter_cargo_clippy(output: &str, exit_code: Option<i32>) -> String {
    let mut warnings = Vec::new();
    let mut current_warning = Vec::new();
    let mut in_warning = false;
    let lines: Vec<&str> = output.lines().collect();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("warning:") || trimmed.starts_with("error:") {
            if in_warning && !current_warning.is_empty() {
                warnings.push(current_warning.join(" "));
                current_warning = Vec::new();
            }
            in_warning = true;
            current_warning.push(trimmed.to_string());
        } else if in_warning
            && (trimmed.starts_with("-->")
                || trimmed.starts_with("= help:")
                || trimmed.starts_with("= note:"))
        {
            current_warning.push(trimmed.to_string());
        }
    }
    if in_warning && !current_warning.is_empty() {
        warnings.push(current_warning.join(" "));
    }

    if warnings.is_empty() {
        return if exit_code == Some(0) {
            "clippy ok".to_string()
        } else {
            generic::truncate_lines(output, 40)
        };
    }

    let grouped = generic::deduplicate_lines(&warnings.join("\n"));
    generic::truncate_lines(&grouped, 80)
}

fn filter_cargo_fmt(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return "fmt ok".to_string();
    }
    let mut files = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.ends_with(".rs") || trimmed.contains("Diff in") {
            files.push(trimmed.to_string());
        }
    }
    if files.is_empty() {
        return format!("{} files need formatting", lines.len());
    }
    format!(
        "{} files need formatting:\n{}",
        files.len(),
        files.join("\n")
    )
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::filters::CommandFilter;

    fn run(
        filter: &dyn CommandFilter,
        subcommand: &str,
        output: &str,
        exit_code: Option<i32>,
    ) -> String {
        let args: Vec<String> = subcommand
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        match filter.filter(&args, output, exit_code) {
            FilterResult::Filtered(s) => s,
            FilterResult::PassThrough(s) => s,
            FilterResult::Silent => String::new(),
        }
    }

    #[test]
    fn test_all_pass() {
        let f = CargoFilter;
        let r = run(
            &f,
            "test",
            "\
running 5 tests
test lib::a ... ok
test lib::b ... ok
test lib::c ... ok
test lib::d ... ok
test lib::e ... ok
test result: ok. 5 passed; 0 failed",
            Some(0),
        );
        assert!(r.contains("PASSED: 5/5"));
    }

    #[test]
    fn test_with_failures() {
        let f = CargoFilter;
        let r = run(
            &f,
            "test",
            "\
running 2 tests
test lib::pass ... ok
test lib::fail ... FAILED

failures:

---- lib::fail ----
panicked at 'assertion failed'

failures:
    lib::fail

test result: FAILED. 1 passed; 1 failed",
            Some(1),
        );
        assert!(r.contains("FAILED: 1/2"));
    }

    #[test]
    fn build_success() {
        let f = CargoFilter;
        let r = run(
            &f,
            "build",
            "Compiling miskin v0.1.0\nFinished dev [unoptimized] target(s) in 2s",
            Some(0),
        );
        assert!(r.contains("Compiling"));
        assert!(r.contains("Finished"));
    }

    #[test]
    fn clippy_ok() {
        let f = CargoFilter;
        let r = run(
            &f,
            "clippy",
            "Checking miskin v0.1.0\nFinished dev",
            Some(0),
        );
        assert!(r.contains("clippy ok"));
    }

    #[test]
    fn fmt_ok() {
        let f = CargoFilter;
        let r = run(&f, "fmt", "", Some(0));
        assert_eq!(r, "fmt ok");
    }
}
