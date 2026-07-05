use crate::filters::generic;
use super::{CommandFilter, FilterResult};

pub struct FilesFilter;

impl CommandFilter for FilesFilter {
    fn name(&self) -> &str {
        "files"
    }

    fn aliases(&self) -> &[&str] {
        &["ls", "cat", "find", "tree", "read"]
    }

    fn filter(&self, args: &[String], output: &str, _exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let command = args.first().map(|s| s.as_str()).unwrap_or("ls");

        match command {
            "ls" => FilterResult::Filtered(filter_ls(&output)),
            "cat" | "read" => FilterResult::Filtered(filter_cat(&output)),
            "find" => FilterResult::Filtered(filter_find(&output)),
            "tree" => FilterResult::Filtered(filter_tree(&output)),
            "head" | "tail" => FilterResult::Filtered(output.trim().to_string()),
            _ => FilterResult::Filtered(generic::truncate_lines(&output, 150)),
        }
    }
}

fn filter_ls(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        return "empty".to_string();
    }

    if lines.first().map(|l| l.contains('/') || l.contains("total ")).unwrap_or(false) {
        let mut dirs = Vec::new();
        let mut files = Vec::new();
        let mut others = Vec::new();

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.ends_with('/') || trimmed.ends_with('/') {
                dirs.push(trimmed.to_string());
            } else if trimmed.contains('.') {
                files.push(trimmed.to_string());
            } else {
                others.push(trimmed.to_string());
            }
        }

        let mut result = Vec::new();
        if !dirs.is_empty() {
            result.push(format!("dirs ({})", dirs.len()));
            for d in dirs.iter().take(10) {
                result.push(format!("  {}", d));
            }
        }
        if !files.is_empty() {
            result.push(format!("files ({})", files.len()));
            let grouped = generic::group_by_extension(&files);
            for g in grouped.iter().take(10) {
                result.push(format!("  {}", g));
            }
        }
        if !others.is_empty() {
            result.push(format!("other ({})", others.len()));
        }

        result.join("\n")
    } else {
        format!("{} entries\n{}", lines.len(), generic::truncate_lines(output, 60))
    }
}

fn filter_cat(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 50 {
        return output.trim().to_string();
    }
    generic::truncate_lines(output, 80)
}

fn filter_find(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return "no matches".to_string();
    }
    if lines.len() <= 30 {
        return lines.join("\n");
    }
    let prefix_grouped = generic::group_by_common_prefix(&lines.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    if prefix_grouped.len() < lines.len() / 2 {
        return prefix_grouped;
    }
    format!("{} files\n{}", lines.len(), generic::truncate_lines(&lines.join("\n"), 60))
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
    fn ls_empty() {
        let f = FilesFilter;
        let r = run(&f, "ls", "", Some(0));
        assert_eq!(r, "empty");
    }

    #[test]
    fn ls_entries() {
        let f = FilesFilter;
        let r = run(&f, "ls", "main.rs\nlib.rs\nCargo.toml", Some(0));
        assert!(r.contains("3 entries"));
    }

    #[test]
    fn cat_truncates_long() {
        let f = FilesFilter;
        let lines = (0..200).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
        let r = run(&f, "cat", &lines, Some(0));
        assert!(r.contains("lines omitted"));
    }

    #[test]
    fn find_no_matches() {
        let f = FilesFilter;
        let r = run(&f, "find", "", Some(0));
        assert_eq!(r, "no matches");
    }
}

fn filter_tree(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let dirs: Vec<&str> = lines.iter().filter(|l| l.contains('/')).copied().collect();
    let files: Vec<&str> = lines.iter().filter(|l| !l.contains('/') && !l.contains("directories")).copied().collect();

    let mut result = Vec::new();
    result.push(format!("{} dirs, {} files", dirs.len(), files.len()));

    let depth_0: Vec<&str> = lines.iter().filter(|l| l.starts_with("├── ") || l.starts_with("└── ")).copied().collect();
    for d in depth_0.iter().take(30) {
        result.push((*d).to_string());
    }
    if depth_0.len() > 30 {
        result.push(format!("... {} more", depth_0.len() - 30));
    }

    result.join("\n")
}
