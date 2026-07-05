use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct DockerFilter;

impl CommandFilter for DockerFilter {
    fn name(&self) -> &str {
        "docker"
    }

    fn aliases(&self) -> &[&str] {
        &["kubectl", "oc"]
    }

    fn filter(&self, args: &[String], output: &str, _exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let command = args.first().map(|s| s.as_str()).unwrap_or("");

        match command {
            "ps" => FilterResult::Filtered(filter_table(
                &output,
                &["CONTAINER ID", "IMAGE", "STATUS", "NAMES"],
            )),
            "images" => FilterResult::Filtered(filter_table(
                &output,
                &["REPOSITORY", "TAG", "IMAGE ID", "SIZE"],
            )),
            "logs" => FilterResult::Filtered(filter_logs(&output)),
            "compose" => FilterResult::Filtered(filter_compose(&output)),
            _ => FilterResult::Filtered(generic::truncate_lines(&output, 80)),
        }
    }
}

fn filter_table(output: &str, _columns: &[&str]) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 1 {
        return "empty".to_string();
    }

    let data_lines: Vec<&str> = lines
        .iter()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .copied()
        .collect();

    if data_lines.len() > 30 {
        return format!("{} entries", data_lines.len());
    }

    format!("{} entries\n{}", data_lines.len(), data_lines.join("\n"))
}

fn filter_logs(output: &str) -> String {
    generic::deduplicate_lines(output)
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
    fn ps_empty() {
        let f = DockerFilter;
        let r = run(&f, "ps", "CONTAINER ID   IMAGE   STATUS", Some(0));
        assert_eq!(r, "empty");
    }

    #[test]
    fn ps_with_containers() {
        let f = DockerFilter;
        let r = run(
            &f,
            "ps",
            "CONTAINER ID   IMAGE         STATUS\nabc123         nginx:latest  Up 2h\ndef456         redis:alpine  Up 5m",
            Some(0),
        );
        assert!(r.contains("2 entries"));
    }

    #[test]
    fn compose_ok() {
        let f = DockerFilter;
        let r = run(&f, "compose", "", Some(0));
        assert_eq!(r, "ok");
    }
}

fn filter_compose(output: &str) -> String {
    if output.trim().is_empty() {
        return "ok".to_string();
    }
    generic::truncate_lines(output, 30)
}
