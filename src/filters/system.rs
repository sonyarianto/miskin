use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct SystemFilter;

impl CommandFilter for SystemFilter {
    fn name(&self) -> &str {
        "system"
    }

    fn aliases(&self) -> &[&str] {
        &[
            "df", "du", "ps", "top", "wc", "env", "which", "uname", "free",
        ]
    }

    fn filter(&self, _args: &[String], output: &str, _exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.len() <= 20 {
            return FilterResult::Filtered(output.trim().to_string());
        }
        FilterResult::Filtered(generic::truncate_lines(&output, 30))
    }
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
    fn short_output_passthrough() {
        let f = SystemFilter;
        let r = run(&f, "df", "Filesystem   Size\n/dev/sda1    100G", Some(0));
        assert!(r.contains("Filesystem"));
    }

    #[test]
    fn long_output_truncated() {
        let f = SystemFilter;
        let lines = (0..50)
            .map(|i| format!("line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let r = run(&f, "ps", &lines, Some(0));
        assert!(r.contains("lines omitted"));
    }
}
