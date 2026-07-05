use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct CurlFilter;

impl CommandFilter for CurlFilter {
    fn name(&self) -> &str {
        "curl"
    }

    fn aliases(&self) -> &[&str] {
        &["wget"]
    }

    fn filter(&self, _args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        if exit_code == Some(0) {
            let len = output.len();
            if len > 5000 {
                return FilterResult::Filtered(format!(
                    "OK ({} bytes). First 5000 chars:\n{}",
                    len,
                    generic::truncate_lines(&output[..5000.min(len)], 30)
                ));
            }
            return FilterResult::Filtered(generic::truncate_lines(output, 50));
        }
        FilterResult::Filtered(generic::truncate_lines(output, 20))
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
    fn truncates_large() {
        let f = CurlFilter;
        let big = "x".repeat(10000);
        let r = run(&f, "curl", &big, Some(0));
        assert!(r.contains("OK"));
        assert!(r.contains("10000 bytes"));
    }

    #[test]
    fn short_passthrough() {
        let f = CurlFilter;
        let r = run(&f, "curl", "{\"ok\": true}", Some(0));
        assert!(r.contains("ok"));
    }
}
