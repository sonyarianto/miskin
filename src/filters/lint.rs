use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct LintFilter;

impl CommandFilter for LintFilter {
    fn name(&self) -> &str {
        "lint"
    }

    fn aliases(&self) -> &[&str] {
        &[
            "eslint",
            "ruff",
            "biome",
            "clippy",
            "golangci-lint",
            "rubocop",
            "prettier",
            "tsc",
            "mypy",
        ]
    }

    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let command = args.first().map(|s| s.as_str()).unwrap_or("lint");

        match command {
            "eslint" | "biome" => FilterResult::Filtered(filter_grouped_linter(&output, exit_code)),
            "ruff" => FilterResult::Filtered(filter_ruff(&output, exit_code)),
            "tsc" | "mypy" => FilterResult::Filtered(filter_typecheck(&output, exit_code)),
            "prettier" => FilterResult::Filtered(filter_prettier(&output, exit_code)),
            "golangci-lint" => FilterResult::Filtered(filter_grouped_linter(&output, exit_code)),
            _ => FilterResult::Filtered(filter_grouped_linter(&output, exit_code)),
        }
    }
}

fn filter_grouped_linter(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if exit_code == Some(0) {
        if lines
            .iter()
            .any(|l| l.contains("problem") || l.contains("error") || l.contains("warning"))
        {
        } else {
            return "lint ok".to_string();
        }
    }

    let mut problems = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains("error")
            || trimmed.contains("warning")
            || trimmed.contains("problem")
            || trimmed.contains("Error")
            || trimmed.contains("Warning")
            || trimmed.matches(':').count() >= 2
        {
            problems.push(trimmed.to_string());
        } else if trimmed.ends_with(".js")
            || trimmed.ends_with(".ts")
            || trimmed.ends_with(".tsx")
            || trimmed.ends_with(".rs")
            || trimmed.ends_with(".py") && trimmed.contains(':')
        {
            problems.push(trimmed.to_string());
        }
    }

    if problems.is_empty() {
        return "lint ok".to_string();
    }

    let grouped = generic::deduplicate_lines(&problems.join("\n"));
    generic::truncate_lines(&grouped, 80)
}

fn filter_ruff(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if exit_code == Some(0) {
        return "ruff ok".to_string();
    }

    let mut by_file: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for line in &lines {
        let trimmed = line.trim();
        if let Some(colon_pos) = trimmed.find(':') {
            let file = trimmed[..colon_pos].to_string();
            let rest = trimmed[colon_pos + 1..].to_string();
            if let Some(colon_pos2) = rest.find(':') {
                let rule = rest[colon_pos2 + 1..].trim().to_string();
                by_file.entry(file).or_default().push(rule);
            }
        }
    }

    let mut result = Vec::new();
    for (file, rules) in &by_file {
        result.push(format!("{}: {}", file, rules.join(", ")));
    }
    result.join("\n")
}

fn filter_typecheck(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if exit_code == Some(0) {
        return "types ok".to_string();
    }

    let mut by_file: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut error_lines = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with(".ts")
            || trimmed.ends_with(".tsx")
            || trimmed.ends_with(".py")
                && !trimmed.contains("error TS")
                && !trimmed.contains("error")
        {
            continue;
        }
        if let Some(paren) = trimmed.find('(') {
            let file = trimmed[..paren].to_string();
            *by_file.entry(file).or_insert(0) += 1;
        }
        error_lines.push(trimmed.to_string());
    }

    let mut result = Vec::new();
    if !by_file.is_empty() {
        let mut sorted: Vec<_> = by_file.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (file, count) in sorted.iter().take(15) {
            result.push(format!("  {} ({} errors)", file, count));
        }
    }
    if result.is_empty() {
        result.extend(error_lines.iter().take(20).map(|s| s.to_string()));
    }

    result.join("\n")
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
    fn lint_ok() {
        let f = LintFilter;
        let r = run(&f, "eslint", "", Some(0));
        assert_eq!(r, "lint ok");
    }

    #[test]
    fn lint_with_errors() {
        let f = LintFilter;
        let r = run(
            &f,
            "eslint",
            "src/main.ts\n  1:5  error  no-unused-vars\n  3:2  warning  semi",
            Some(1),
        );
        assert!(r.contains("error"));
    }

    #[test]
    fn ruff_ok() {
        let f = LintFilter;
        let r = run(&f, "ruff", "", Some(0));
        assert_eq!(r, "ruff ok");
    }

    #[test]
    fn tsc_ok() {
        let f = LintFilter;
        let r = run(&f, "tsc", "", Some(0));
        assert_eq!(r, "types ok");
    }
}

fn filter_prettier(output: &str, exit_code: Option<i32>) -> String {
    if exit_code == Some(0) {
        return "format ok".to_string();
    }
    let lines: Vec<&str> = output
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.contains("Checking"))
        .collect();
    if lines.is_empty() {
        return "format ok".to_string();
    }
    format!(
        "{} files need formatting:\n{}",
        lines.len(),
        generic::truncate_lines(&lines.join("\n"), 30)
    )
}
