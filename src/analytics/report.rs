use super::AnalyticsStore;

pub fn format_summary(store: &AnalyticsStore) -> String {
    if store.command_count == 0 {
        return "No data yet. Run some commands through miskin first.".to_string();
    }

    format!(
        "Miskin Token Savings Report\n\
         ==========================\n\
         Commands processed : {:>6}\n\
         Original tokens    : {:>6}\n\
         Filtered tokens    : {:>6}\n\
         Tokens saved       : {:>6}\n\
         Savings rate       : {:>5.1}%\n",
        store.command_count,
        store.total_original,
        store.total_filtered,
        store.total_saved(),
        store.savings_pct()
    )
}

pub fn format_graph(store: &AnalyticsStore) -> String {
    if store.entries.is_empty() {
        return "No data.".to_string();
    }

    let recent: Vec<_> = store.entries.iter().rev().take(20).collect();
    let mut out = String::from("Last 20 commands:\n");

    for entry in recent.iter().rev() {
        let bar_len = if entry.original_tokens > 0 {
            ((entry.filtered_tokens as f64 / entry.original_tokens as f64) * 20.0) as usize
        } else {
            20
        };
        let bar = "█".repeat(bar_len.min(20));
        let empty = "░".repeat(20 - bar_len.min(20));
        out.push_str(&format!(
            "  {:<20} {}{} {:>4}→{:>4}\n",
            truncate(&entry.command, 20),
            bar,
            empty,
            entry.original_tokens,
            entry.filtered_tokens
        ));
    }

    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
