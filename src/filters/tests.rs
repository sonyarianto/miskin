use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct TestsFilter;

impl CommandFilter for TestsFilter {
    fn name(&self) -> &str {
        "tests"
    }

    fn aliases(&self) -> &[&str] {
        &["pytest", "jest", "vitest", "go", "rspec", "playwright"]
    }

    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let command = args.first().map(|s| s.as_str()).unwrap_or("tests");

        match command {
            "pytest" => FilterResult::Filtered(filter_pytest(&output, exit_code)),
            "jest" | "vitest" => FilterResult::Filtered(filter_jest(&output, exit_code)),
            "go" => FilterResult::Filtered(filter_go_test(&output, exit_code)),
            "rspec" => FilterResult::Filtered(filter_rspec(&output, exit_code)),
            "playwright" => FilterResult::Filtered(filter_playwright(&output, exit_code)),
            _ => FilterResult::Filtered(filter_generic(&output, exit_code)),
        }
    }
}

fn filter_pytest(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut pass_count = 0u32;
    let mut fail_count = 0u32;
    let mut failures = Vec::new();
    let mut in_failure = false;
    let mut current_failure = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.ends_with(" PASSED") || trimmed.ends_with(" PASSED  [") {
            pass_count += 1;
        } else if trimmed.ends_with(" FAILED") || trimmed.ends_with(" FAILED  [") {
            fail_count += 1;
            in_failure = true;
            let name = trimmed.split_whitespace().next().unwrap_or(trimmed);
            current_failure.push(name.to_string());
        } else if in_failure
            && (trimmed.starts_with("E ")
                || trimmed.starts_with("> ")
                || trimmed.starts_with("AssertionError")
                || trimmed.starts_with("assert "))
        {
            current_failure.push(trimmed.to_string());
        } else if in_failure && trimmed.is_empty() {
            if !current_failure.is_empty() {
                failures.push(current_failure.join(" | "));
                current_failure = Vec::new();
            }
            in_failure = false;
        }
    }

    if exit_code == Some(0) {
        return format!("PASSED: {} tests", pass_count);
    }

    let mut result = vec![format!(
        "FAILED: {}/{}\n",
        fail_count,
        pass_count + fail_count
    )];
    for f in failures.iter().take(15) {
        result.push(f.clone());
    }
    result.join("\n")
}

fn filter_jest(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut pass_count = 0u32;
    let mut fail_count = 0u32;
    let mut suite_count = 0u32;
    let mut failures = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("PASS ") || trimmed.starts_with("\u{2713}") {
            pass_count += 1;
            suite_count += 1;
        } else if trimmed.starts_with("FAIL ") || trimmed.starts_with("\u{2715}") {
            fail_count += 1;
            failures.push(trimmed.to_string());
        } else if trimmed.starts_with("Tests:") && trimmed.contains("failed") {
            let parts: Vec<&str> = trimmed.split(',').collect();
            for p in parts {
                let num: String = p.chars().filter(|c| c.is_ascii_digit()).collect();
                if let Ok(n) = num.parse::<u32>() {
                    if p.contains("failed") {
                        fail_count = n;
                    } else if p.contains("passed") {
                        pass_count = n;
                    }
                }
            }
        }
    }

    if fail_count == 0 {
        return format!(
            "PASSED: {}/{} tests ({} suites)",
            pass_count, pass_count, suite_count
        );
    }

    format!(
        "FAILED: {}/{}\n{}",
        fail_count,
        pass_count + fail_count,
        generic::truncate_lines(&failures.join("\n"), 30)
    )
}

fn filter_go_test(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut packages = Vec::new();
    let mut failures = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("ok ") || trimmed.starts_with("? ") || trimmed.starts_with("FAIL ") {
            packages.push(trimmed.to_string());
        }
        if trimmed.starts_with("--- FAIL") {
            failures.push(trimmed.to_string());
        }
    }

    if failures.is_empty() {
        return format!("ok  {} packages", packages.len());
    }

    format!(
        "{} packages, {} failed\n{}",
        packages.len(),
        failures.len(),
        generic::truncate_lines(&failures.join("\n"), 30)
    )
}

fn filter_rspec(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut summary = Vec::new();
    let mut found_summary = false;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("Failures:")
            || trimmed.starts_with("Finished in")
            || trimmed.starts_with("examples")
        {
            found_summary = true;
        }
        if found_summary {
            summary.push(trimmed.to_string());
        }
    }

    if !summary.is_empty() {
        return summary.join("\n");
    }
    "all examples passed".to_string()
}

fn filter_playwright(output: &str, _exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut pass_count = 0u32;
    let mut fail_count = 0u32;
    let mut failures = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("passed") && trimmed.contains('(') && trimmed.contains(')') {
            let num: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = num.parse::<u32>() {
                pass_count = n;
            }
        }
        if trimmed.contains("failed") && trimmed.contains('(') && trimmed.contains(')') {
            let num: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = num.parse::<u32>() {
                fail_count = n;
            }
        }
        if trimmed.contains("Error:") {
            failures.push(trimmed.to_string());
        }
    }

    if fail_count == 0 {
        return format!("PASSED: {} tests", pass_count);
    }
    format!(
        "FAILED: {}/{}\n{}",
        fail_count,
        pass_count + fail_count,
        generic::truncate_lines(&failures.join("\n"), 30)
    )
}

fn filter_generic(output: &str, exit_code: Option<i32>) -> String {
    if exit_code == Some(0) {
        return "ok".to_string();
    }
    let lines: Vec<&str> = output.lines().collect();
    let failures: Vec<&str> = lines
        .iter()
        .filter(|l| {
            l.contains("FAIL") || l.contains("fail") || l.contains("error") || l.contains("Error")
        })
        .copied()
        .collect();

    if failures.is_empty() {
        return generic::truncate_lines(output, 40);
    }
    generic::truncate_lines(&failures.join("\n"), 40)
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
    fn pytest_all_pass() {
        let f = TestsFilter;
        let result = run(
            &f,
            "pytest",
            "\
tests/test_foo.py::test_one PASSED
tests/test_foo.py::test_two PASSED
========================= 2 passed in 0.42s =========================",
            Some(0),
        );
        assert!(result.contains("PASSED: 2 tests"));
    }

    #[test]
    fn jest_pass() {
        let f = TestsFilter;
        let result = run(
            &f,
            "jest",
            "\
PASS  src/__tests__/foo.test.ts
PASS  src/__tests__/bar.test.ts
Tests: 5 passed, 5 total",
            Some(0),
        );
        assert!(result.contains("PASSED:"));
    }

    #[test]
    fn go_test_ok() {
        let f = TestsFilter;
        let result = run(&f, "go", "ok  \tgithub.com/user/pkg\t0.123s", Some(0));
        assert!(result.contains("ok  "));
        assert!(result.contains("1 packages"));
    }

    #[test]
    fn generic_runner_pass() {
        let f = TestsFilter;
        let result = run(&f, "unknown", "all tests passed", Some(0));
        assert_eq!(result, "ok");
    }
}
