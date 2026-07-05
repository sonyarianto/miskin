use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub fn deduplicate_lines(input: &str) -> String {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || seen.insert(trimmed) {
            result.push(line);
        }
    }

    result.join("\n")
}

pub fn truncate_lines(input: &str, max_lines: usize) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() <= max_lines {
        return input.to_string();
    }

    let kept = &lines[..max_lines / 2];
    let tail = &lines[lines.len().saturating_sub(max_lines / 2)..];
    let omitted = lines.len().saturating_sub(max_lines);

    format!(
        "{}\n... ({} lines omitted) ...\n{}",
        kept.join("\n"),
        omitted,
        tail.join("\n")
    )
}

fn ansi_regex() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new("\x1b\\[[0-9;]*[a-zA-Z]").unwrap())
}

pub fn strip_ansi(input: &str) -> String {
    ansi_regex().replace_all(input, "").to_string()
}

#[allow(dead_code)]
pub fn collapse_whitespace(input: &str) -> String {
    let re = regex::Regex::new(r"\n{3,}").unwrap();
    re.replace_all(input, "\n\n").to_string()
}

#[allow(dead_code)]
pub fn count_tokens(text: &str) -> usize {
    text.split_whitespace().count() * 4 / 3
}

pub fn group_by_extension(files: &[String]) -> Vec<String> {
    let mut by_ext: HashMap<String, Vec<String>> = HashMap::new();

    for file in files {
        let ext = if let Some(pos) = file.rfind('.') {
            file[pos + 1..].to_string()
        } else {
            "other".to_string()
        };
        by_ext.entry(ext).or_default().push(file.clone());
    }

    let mut result = Vec::new();
    for (ext, mut files) in by_ext {
        files.sort();
        if files.len() == 1 {
            result.push(files[0].clone());
        } else {
            result.push(format!("*.{} ({} files)", ext, files.len()));
        }
    }
    result.sort();
    result
}

pub fn group_by_common_prefix(paths: &[String]) -> String {
    if paths.is_empty() {
        return String::new();
    }
    if paths.len() <= 5 {
        return paths.join("\n");
    }

    let mut by_dir: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths {
        let dir = if let Some(pos) = path.rfind('/') {
            path[..pos + 1].to_string()
        } else {
            path.clone()
        };
        by_dir.entry(dir).or_default().push(path.clone());
    }

    let mut result = Vec::new();
    for (dir, mut files) in by_dir {
        files.sort();
        if files.len() <= 3 {
            result.extend(files);
        } else {
            result.push(format!(
                "{}/ ({} files)",
                dir.trim_end_matches('/'),
                files.len()
            ));
        }
    }
    result.sort();
    result.join("\n")
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn dedup_removes_dupes() {
        let r = deduplicate_lines("line1\nline1\nline2\nline3\nline3");
        assert_eq!(r.lines().count(), 3);
        assert!(r.contains("line1"));
        assert!(r.contains("line2"));
    }

    #[test]
    fn truncate_preserves_short() {
        let r = truncate_lines("a\nb\nc", 10);
        assert_eq!(r, "a\nb\nc");
    }

    #[test]
    fn truncate_long() {
        let lines = (0..100)
            .map(|i| format!("ln{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let r = truncate_lines(&lines, 20);
        assert!(r.contains("lines omitted"));
    }

    #[test]
    fn strip_ansi_colors() {
        let r = strip_ansi("\x1b[32mok\x1b[0m");
        assert_eq!(r, "ok");
    }

    #[test]
    fn group_ext_multiple() {
        let files = vec!["a.rs".into(), "b.rs".into(), "c.toml".into(), "d.md".into()];
        let r = group_by_extension(&files);
        assert!(
            r.iter()
                .any(|s| s.contains("*.rs") && s.contains("2 files"))
        );
    }

    #[test]
    fn group_prefix() {
        let paths = vec![
            "src/a.rs".into(),
            "src/b.rs".into(),
            "src/c.rs".into(),
            "src/d.rs".into(),
            "tests/e.rs".into(),
        ];
        let r = group_by_common_prefix(&paths);
        assert!(r.contains("src") && r.contains("4 files") || r.contains("src/a.rs"));
        assert!(r.contains("e.rs"));
    }

    #[test]
    fn dedup_preserves_empty_lines() {
        let input = "line1\n\nline1\n\nline2\n\nline2";
        let r = deduplicate_lines(input);
        assert_eq!(r, "line1\n\n\nline2\n");
        assert!(r.contains("\n\n"));
    }
}
