use super::{CommandFilter, FilterResult};
use crate::filters::generic;

pub struct GitFilter;

impl CommandFilter for GitFilter {
    fn name(&self) -> &str {
        "git"
    }

    fn filter(&self, args: &[String], output: &str, exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let subcommand = args.first().map(|s| s.as_str()).unwrap_or("");

        if exit_code == Some(0) {
            match subcommand {
                "add" | "commit" | "push" | "pull" | "fetch" | "checkout" | "switch" | "merge"
                | "rebase" | "tag" | "stash" | "reset" | "restore" | "rm" | "mv" | "clean"
                | "init" | "clone" | "remote" => {
                    return FilterResult::Filtered(filter_git_ok(subcommand, &output));
                }
                "status" => return FilterResult::Filtered(filter_git_status(&output)),
                "diff" => return FilterResult::Filtered(filter_git_diff(&output)),
                "log" => return FilterResult::Filtered(filter_git_log(&output)),
                "branch" => return FilterResult::Filtered(filter_git_branch(&output)),
                _ => {}
            }
        }

        FilterResult::Filtered(output.trim().to_string())
    }
}

fn filter_git_ok(subcommand: &str, output: &str) -> String {
    if subcommand == "push" || subcommand == "pull" || subcommand == "fetch" {
        let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            return format!("ok {}", subcommand);
        }
        let last = lines.last().unwrap_or(&"");
        let branch = last.split('/').next_back().unwrap_or(".");
        return format!("ok {} {}", subcommand, branch);
    }

    if subcommand == "commit" {
        for line in output.lines() {
            if let Some(pos) = line.find(']') {
                let inside = &line[..pos + 1];
                if inside.starts_with('[') {
                    return inside.to_string();
                }
            }
        }
        return "ok commit".to_string();
    }

    if subcommand == "branch" {
        let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
        let current = lines.iter().find(|l| l.starts_with('*'));
        if let Some(cur) = current {
            let name = cur.trim_start_matches("* ").trim();
            return format!("* {} (+{} branches)", name, lines.len() - 1);
        }
        return format!("{} branches", lines.len());
    }

    format!("ok {}", subcommand)
}

fn filter_git_status(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.is_empty() {
        return "clean".to_string();
    }

    let mut branch = String::new();
    let mut changed = Vec::new();
    let mut untracked_files = Vec::new();
    let mut current_section = "";

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("On branch ") || trimmed.starts_with("HEAD detached") {
            branch = trimmed.to_string();
            continue;
        }

        if trimmed.starts_with("Changes to be committed:") {
            current_section = "staged";
            continue;
        } else if trimmed.starts_with("Changes not staged for commit:") {
            current_section = "unstaged";
            continue;
        } else if trimmed.starts_with("Untracked files:") {
            current_section = "untracked";
            continue;
        }

        if trimmed.starts_with("(use ")
            || trimmed.starts_with("no changes")
            || trimmed.starts_with("nothing")
        {
            continue;
        }

        if trimmed.starts_with("modified:")
            || trimmed.starts_with("new file:")
            || trimmed.starts_with("deleted:")
            || trimmed.starts_with("renamed:")
        {
            changed.push(format!("  [{}] {}", current_section, trimmed));
        } else if !trimmed.starts_with("  (") {
            let path = trimmed.trim_start_matches("\"").trim_end_matches("\"");
            untracked_files.push(path.to_string());
        }
    }

    let mut result = Vec::new();
    if !branch.is_empty() {
        result.push(branch);
    }

    if !changed.is_empty() {
        result.push(format!("Changes ({}):", changed.len()));
        for c in changed.iter().take(20) {
            result.push(c.clone());
        }
        if changed.len() > 20 {
            result.push(format!("  ... and {} more changes", changed.len() - 20));
        }
    }

    if !untracked_files.is_empty() {
        result.push(format!("Untracked ({}):", untracked_files.len()));
        if untracked_files.len() <= 10 {
            for f in &untracked_files {
                result.push(format!("  {}", f));
            }
        } else {
            let prefix_grouped = generic::group_by_common_prefix(&untracked_files);
            result.push(prefix_grouped);
        }
    }

    if result.is_empty() {
        return "clean".to_string();
    }

    result.join("\n")
}

fn filter_git_diff(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut file_count = 0u32;
    let mut additions = 0u32;
    let mut deletions = 0u32;
    let mut current_file = String::new();
    let mut file_changes = Vec::new();
    let mut file_additions = 0u32;
    let mut file_deletions = 0u32;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("diff --git") {
            if file_count > 0 && (file_additions > 0 || file_deletions > 0) {
                file_changes.push(format!(
                    "  {} (+{}/-{})",
                    current_file, file_additions, file_deletions
                ));
            }
            file_count += 1;
            file_additions = 0;
            file_deletions = 0;
            current_file = String::new();
            continue;
        }
        if trimmed.starts_with("---") || trimmed.starts_with("+++") {
            if trimmed.starts_with("+++ b/") {
                current_file = trimmed.trim_start_matches("+++ b/").to_string();
            }
            if file_count == 0 {
                file_count = 1;
            }
            continue;
        }
        if trimmed.starts_with('+') && !trimmed.starts_with("+++") {
            additions += 1;
            file_additions += 1;
        } else if trimmed.starts_with('-') && !trimmed.starts_with("---") {
            deletions += 1;
            file_deletions += 1;
        }
    }

    if file_count > 0 && (file_additions > 0 || file_deletions > 0) {
        file_changes.push(format!(
            "  {} (+{}/-{})",
            current_file, file_additions, file_deletions
        ));
    }

    let mut result = vec![format!(
        "{} files (+{}/-{})",
        file_count, additions, deletions
    )];

    for fc in file_changes.iter().take(15) {
        result.push(fc.clone());
    }
    if file_changes.len() > 15 {
        result.push(format!("  ... and {} more files", file_changes.len() - 15));
    }

    result.join("\n")
}

fn filter_git_log(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let commits: Vec<&str> = lines.to_vec();

    if commits.len() > 20 {
        format!(
            "{}\n... ({} more commits)",
            commits[..20].join("\n"),
            commits.len() - 20
        )
    } else {
        commits.join("\n")
    }
}

fn filter_git_branch(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let current = lines.iter().find(|l| l.starts_with('*'));
    match current {
        Some(cur) => {
            let name = cur.trim_start_matches("* ").trim();
            format!("* {} (+{} branches)", name, lines.len() - 1)
        }
        None => format!("{} branches", lines.len()),
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
    fn status_clean() {
        let f = GitFilter;
        let r = run(
            &f,
            "status",
            "On branch main\nnothing to commit, working tree clean",
            Some(0),
        );
        assert!(r.contains("On branch main"));
    }

    #[test]
    fn status_modified() {
        let f = GitFilter;
        let r = run(
            &f,
            "status",
            "On branch main\nChanges not staged for commit:\n\tmodified:   src/main.rs",
            Some(0),
        );
        assert!(r.contains("On branch main"));
        assert!(r.contains("modified:"));
    }

    #[test]
    fn diff_compact() {
        let f = GitFilter;
        let r = run(
            &f,
            "diff",
            "diff --git a/src/main.rs b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n-old\n+new",
            Some(0),
        );
        assert!(r.contains("1 files"));
    }

    #[test]
    fn ok_add() {
        let f = GitFilter;
        let r = run(&f, "add", "", Some(0));
        assert_eq!(r, "ok add");
    }

    #[test]
    fn ok_commit() {
        let f = GitFilter;
        let r = run(
            &f,
            "commit",
            "[main abc1234] fix: bug\n 1 file changed",
            Some(0),
        );
        assert!(r.contains("[main abc1234]"));
    }

    #[test]
    fn ok_push() {
        let f = GitFilter;
        let r = run(
            &f,
            "push",
            "To github.com:user/repo\n   abc..def  main -> main",
            Some(0),
        );
        assert!(r.contains("ok push"));
    }

    #[test]
    fn branch_current() {
        let f = GitFilter;
        let r = run(&f, "branch", "* main\n  feat/x\n  bugfix/y", Some(0));
        assert!(r.contains("* main"));
        assert!(r.contains("2 branches"));
    }

    #[test]
    fn log_truncates() {
        let f = GitFilter;
        let lines: Vec<String> = (0..25).map(|i| format!("abc{} commit {}", i, i)).collect();
        let r = run(&f, "log", &lines.join("\n"), Some(0));
        assert!(r.contains("more commits"));
    }
}
