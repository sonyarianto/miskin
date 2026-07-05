use crate::filters::generic;
use super::{CommandFilter, FilterResult};

pub struct NpmFilter;

impl CommandFilter for NpmFilter {
    fn name(&self) -> &str {
        "npm"
    }

    fn aliases(&self) -> &[&str] {
        &["pnpm", "yarn", "npx", "bun", "pip", "uv", "bundle", "gem"]
    }

    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let command = args.first().map(|s| s.as_str()).unwrap_or("run");

        match command {
            "install" | "i" | "add" => FilterResult::Filtered(filter_install(&output, exit_code)),
            "run" | "test" | "exec" => FilterResult::Filtered(filter_script(&output, exit_code)),
            "list" | "ls" => FilterResult::Filtered(filter_list(&output)),
            "outdated" => FilterResult::Filtered(filter_outdated(&output)),
            _ => FilterResult::Filtered(generic::truncate_lines(&output, 60)),
        }
    }
}

fn filter_install(output: &str, exit_code: Option<i32>) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let added: Vec<&str> = lines.iter().filter(|l| l.contains("added")).copied().collect();
    let removed: Vec<&str> = lines.iter().filter(|l| l.contains("removed")).copied().collect();
    let changed: Vec<&str> = lines.iter().filter(|l| l.contains("changed") || l.contains("updated")).copied().collect();

    for line in &added {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    for line in &removed {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    for line in &changed {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    if exit_code == Some(0) {
        return "install ok".to_string();
    }
    generic::truncate_lines(&output, 40)
}

fn filter_script(output: &str, exit_code: Option<i32>) -> String {
    if exit_code == Some(0) {
        let lines: Vec<&str> = output.lines().collect();
        let last_few: Vec<&str> = lines.iter().rev().take(3).copied().collect();
        if last_few.is_empty() {
            return "ok".to_string();
        }
        return last_few.into_iter().rev().collect::<Vec<_>>().join("\n");
    }
    generic::truncate_lines(&output, 50)
}

fn filter_list(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let packages: Vec<&str> = lines.iter().filter(|l| l.contains('@')).copied().collect();
    if packages.is_empty() {
        return "no packages".to_string();
    }
    format!("{} packages\n{}", packages.len(), generic::truncate_lines(&packages.join("\n"), 40))
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::filters::CommandFilter;

    fn run(filter: &dyn CommandFilter, subcommand: &str, output: &str, exit_code: Option<i32>) -> String {
        let args: Vec<String> = subcommand.split_whitespace().map(|s| s.to_string()).collect();
        match filter.filter(&args, output, exit_code) {
            FilterResult::Filtered(s) => s,
            FilterResult::PassThrough(s) => s,
            FilterResult::Silent => String::new(),
        }
    }

    #[test]
    fn install_ok() {
        let f = NpmFilter;
        let r = run(&f, "install", "added 42 packages in 2s", Some(0));
        assert!(r.contains("added 42 packages"));
    }

    #[test]
    fn install_ok_yarn() {
        let f = NpmFilter;
        let r = run(&f, "install", "Done in 3s.", Some(0));
        assert_eq!(r, "install ok");
    }

    #[test]
    fn list() {
        let f = NpmFilter;
        let r = run(&f, "list", "react@18.2.0\nlodash@4.17.1\nvue@3.4.0", Some(0));
        assert!(r.contains("3 packages"));
    }

    #[test]
    fn outdated_all_fresh() {
        let f = NpmFilter;
        let r = run(&f, "outdated", "", Some(0));
        assert_eq!(r, "all up to date");
    }
}

fn filter_outdated(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let outdated: Vec<&str> = lines.iter().filter(|l| l.contains("->") || l.contains("→")).copied().collect();
    if outdated.is_empty() {
        return "all up to date".to_string();
    }
    format!("{} outdated\n{}", outdated.len(), outdated.join("\n"))
}
