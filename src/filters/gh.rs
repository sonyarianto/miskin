use crate::filters::generic;
use super::{CommandFilter, FilterResult};

pub struct GhFilter;

impl CommandFilter for GhFilter {
    fn name(&self) -> &str {
        "gh"
    }

    fn filter(&self, args: &[String], output: &str, _exit_code: Option<i32>) -> FilterResult {
        let output = generic::strip_ansi(output);
        let subcommand = args.first().map(|s| s.as_str()).unwrap_or("");

        match subcommand {
            "pr" => {
                let action = args.get(1).map(|s| s.as_str()).unwrap_or("list");
                match action {
                    "list" => FilterResult::Filtered(filter_pr_list(&output)),
                    "view" => FilterResult::Filtered(filter_pr_view(&output)),
                    "status" => FilterResult::Filtered(filter_pr_status(&output)),
                    _ => FilterResult::Filtered(generic::truncate_lines(&output, 50)),
                }
            }
            "issue" => {
                let action = args.get(1).map(|s| s.as_str()).unwrap_or("list");
                match action {
                    "list" => FilterResult::Filtered(filter_issue_list(&output)),
                    _ => FilterResult::Filtered(generic::truncate_lines(&output, 50)),
                }
            }
            "run" => {
                let action = args.get(1).map(|s| s.as_str()).unwrap_or("list");
                match action {
                    "list" => FilterResult::Filtered(filter_run_list(&output)),
                    _ => FilterResult::Filtered(generic::truncate_lines(&output, 50)),
                }
            }
            "repo" => FilterResult::Filtered(filter_repo_view(&output)),
            "auth" => FilterResult::Filtered(filter_auth(&output)),
            _ => FilterResult::Filtered(generic::truncate_lines(&output, 60)),
        }
    }
}

fn filter_pr_list(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.len() <= 1 {
        return "no open PRs".to_string();
    }

    let _header = lines.first().map(|s| s.to_string()).unwrap_or_default();
    let data: Vec<&str> = lines.iter().skip(1).copied().collect();

    if data.is_empty() {
        return "no open PRs".to_string();
    }

    let mut prs = Vec::new();
    for line in &data {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let number = parts[0];
            let title_start = parts[1..]
                .iter()
                .position(|w| !w.is_empty());
            let title = if let Some(pos) = title_start {
                parts[1 + pos..].join(" ")
            } else {
                parts[1..].join(" ")
            };
            let short_title = if title.len() > 60 {
                format!("{}...", &title[..57])
            } else {
                title
            };
            prs.push(format!("  #{} {}", number, short_title));
        } else {
            prs.push(format!("  {}", line));
        }
    }

    format!("{} PRs\n{}", data.len(), prs.join("\n"))
}

fn filter_pr_view(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("title:") || trimmed.starts_with("state:")
            || trimmed.starts_with("author:") || trimmed.starts_with("labels:")
            || trimmed.starts_with("additions:") || trimmed.starts_with("deletions:")
            || trimmed.starts_with("merged:") || trimmed.starts_with("draft:")
            || trimmed.starts_with("base:") || trimmed.starts_with("head:")
            || trimmed.starts_with("url:") || trimmed.starts_with("body:")
            || trimmed.starts_with("Milestone:") || trimmed.starts_with("Assignees:")
        {
            result.push(trimmed.to_string());
        } else if trimmed.contains("✓") || trimmed.contains("✗")
            || trimmed.contains("PASS") || trimmed.contains("FAIL")
        {
            result.push(trimmed.to_string());
        }
    }

    if result.is_empty() {
        return generic::truncate_lines(output, 30);
    }
    result.join("\n")
}

fn filter_pr_status(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        return "no PRs".to_string();
    }

    let mut mine = Vec::new();
    let mut review = Vec::new();
    let mut current = &mut mine;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("Relevant") {
            continue;
        }
        if trimmed.starts_with("Current branch") {
            current = &mut review;
            continue;
        }
        if trimmed.starts_with("Created by you") {
            current = &mut mine;
            continue;
        }
        if trimmed.starts_with("Requesting a code review from you") {
            current = &mut review;
        }
        if !trimmed.is_empty() && !trimmed.starts_with('-') && !trimmed.starts_with('#') {
            if let Some(_hash_pos) = trimmed.find('#') {
                let pr_info = trimmed.trim_start_matches(|c: char| !c.is_ascii_digit() && c != '#');
                if pr_info.starts_with('#') {
                    current.push(pr_info.to_string());
                }
            }
        }
    }

    let mut out = Vec::new();
    if !mine.is_empty() {
        out.push(format!("My PRs ({}):", mine.len()));
        for pr in &mine {
            out.push(format!("  {}", pr));
        }
    }
    if !review.is_empty() {
        out.push(format!("Needs review ({}):", review.len()));
        for pr in &review {
            out.push(format!("  {}", pr));
        }
    }

    out.join("\n")
}

fn filter_issue_list(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.len() <= 1 {
        return "no open issues".to_string();
    }

    let data: Vec<&str> = lines.iter().skip(1).copied().collect();
    let mut issues = Vec::new();

    for line in &data {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let number = parts[0];
            let title = parts[1..].join(" ");
            let short = if title.len() > 70 {
                format!("{}...", &title[..67])
            } else {
                title
            };
            issues.push(format!("  #{} {}", number, short));
        } else {
            issues.push(format!("  {}", line));
        }
    }

    format!("{} issues\n{}", data.len(), issues.join("\n"))
}

fn filter_run_list(output: &str) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.len() <= 1 {
        return "no workflow runs".to_string();
    }

    let data: Vec<&str> = lines.iter().skip(1).copied().collect();

    let mut completed = Vec::new();
    let mut failed = Vec::new();
    let mut in_progress = Vec::new();

    for line in &data {
        let trimmed = line.trim();
        if trimmed.contains("failure") || trimmed.contains("failed") || trimmed.contains("✗") {
            failed.push(trimmed.to_string());
        } else if trimmed.contains("in_progress") || trimmed.contains("pending") || trimmed.contains("queued") {
            in_progress.push(trimmed.to_string());
        } else if trimmed.contains("completed") || trimmed.contains("success") || trimmed.contains("✓") {
            completed.push(trimmed.to_string());
        } else {
            completed.push(trimmed.to_string());
        }
    }

    let mut out = vec![format!("{} runs", data.len())];
    if !failed.is_empty() {
        out.push(format!("  ✗ {} failed:", failed.len()));
        for f in failed.iter().take(5) {
            out.push(format!("    {}", f));
        }
    }
    if !in_progress.is_empty() {
        out.push(format!("  ○ {} running:", in_progress.len()));
        for r in in_progress.iter().take(3) {
            out.push(format!("    {}", r));
        }
    }
    if !completed.is_empty() {
        out.push(format!("  ✓ {} passed", completed.len()));
    }

    out.join("\n")
}

fn filter_repo_view(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("name:") || trimmed.starts_with("description:")
            || trimmed.starts_with("visibility:") || trimmed.starts_with("default branch:")
            || trimmed.starts_with("stars:") || trimmed.starts_with("forks:")
            || trimmed.starts_with("open issues:") || trimmed.starts_with("open PRs:")
        {
            result.push(trimmed.to_string());
        }
    }

    if result.is_empty() {
        return generic::truncate_lines(output, 20);
    }
    result.join("\n")
}

fn filter_auth(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("Logged in to") {
            return trimmed.to_string();
        }
    }
    "auth ok".to_string()
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::filters::CommandFilter;

    fn run(filter: &dyn CommandFilter, args_str: &str, output: &str, exit_code: Option<i32>) -> String {
        let args: Vec<String> = args_str.split_whitespace().map(|s| s.to_string()).collect();
        match filter.filter(&args, output, exit_code) {
            FilterResult::Filtered(s) => s,
            FilterResult::PassThrough(s) => s,
            FilterResult::Silent => String::new(),
        }
    }

    #[test]
    fn pr_list_empty() {
        let f = GhFilter;
        let r = run(&f, "pr list", "No open pull requests", Some(0));
        assert_eq!(r, "no open PRs");
    }

    #[test]
    fn pr_list_with_results() {
        let f = GhFilter;
        let r = run(&f, "pr list", "NUMBER\tTITLE\tBRANCH\n123\tFix auth bug\tfix/auth\n456\tAdd tests\tfeat/tests", Some(0));
        assert!(r.contains("2 PRs"));
        assert!(r.contains("#123"));
        assert!(r.contains("#456"));
    }

    #[test]
    fn issue_list_empty() {
        let f = GhFilter;
        let r = run(&f, "issue list", "No open issues", Some(0));
        assert_eq!(r, "no open issues");
    }

    #[test]
    fn run_list() {
        let f = GhFilter;
        let r = run(&f, "run list", "STATUS\tNAME\tBRANCH\ncompleted\tCI\tmain\nfailed\tDeploy\tmain\nin_progress\tRelease\tmain", Some(0));
        assert!(r.contains("3 runs"));
        assert!(r.contains("1 failed"));
        assert!(r.contains("1 running"));
        assert!(r.contains("1 passed"));
    }

    #[test]
    fn auth_ok() {
        let f = GhFilter;
        let r = run(&f, "auth", "Logged in to github.com as sony", Some(0));
        assert!(r.contains("Logged in to"));
    }
}
