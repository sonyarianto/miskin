use crate::analytics::{AnalyticsStore, report};
use crate::config::MiskinConfig;
use std::collections::HashMap;

pub fn run(graph: bool, history: bool, daily: bool, _all: bool, json: bool) -> anyhow::Result<()> {
    let config = MiskinConfig::load()?;
    let store = AnalyticsStore::load(&config.analytics.data_dir).unwrap_or_default();

    if store.entries.is_empty() {
        if json {
            println!(
                "{{\"commands\": 0, \"original_tokens\": 0, \"filtered_tokens\": 0, \"savings_pct\": 0.0}}"
            );
        } else {
            println!("No data yet. Run some commands through miskin first.");
        }
        return Ok(());
    }

    if json {
        println!("{{");
        println!("  \"commands\": {},", store.command_count);
        println!("  \"original_tokens\": {},", store.total_original);
        println!("  \"filtered_tokens\": {},", store.total_filtered);
        println!("  \"tokens_saved\": {},", store.total_saved());
        println!("  \"savings_pct\": {:.1}", store.savings_pct());

        if daily {
            print_daily_json(&store);
        }
        if history {
            print_history_json(&store);
        }

        println!("}}");
        return Ok(());
    }

    if daily {
        print_daily_report(&store);
        return Ok(());
    }

    println!("{}", report::format_summary(&store));

    if graph {
        println!();
        println!("{}", report::format_graph(&store));
    }

    if history {
        println!();
        println!("Recent commands:");
        for entry in store.entries.iter().rev().take(20) {
            let pct = if entry.original_tokens > 0 {
                ((entry.original_tokens - entry.filtered_tokens) as f64
                    / entry.original_tokens as f64
                    * 100.0) as u32
            } else {
                0
            };
            println!(
                "  {}  {:<30} {:>4}→{:>4} tokens ({}%)",
                entry.timestamp.format("%m-%d %H:%M"),
                truncate(&entry.command, 30),
                entry.original_tokens,
                entry.filtered_tokens,
                pct,
            );
        }
    }

    Ok(())
}

fn print_daily_report(store: &AnalyticsStore) {
    let mut by_day: HashMap<String, (u64, u64, u64)> = HashMap::new();

    for entry in &store.entries {
        let day = entry.timestamp.format("%Y-%m-%d").to_string();
        let stats = by_day.entry(day).or_insert((0, 0, 0));
        stats.0 += 1;
        stats.1 += entry.original_tokens;
        stats.2 += entry.filtered_tokens;
    }

    let mut days: Vec<_> = by_day.iter().collect();
    days.sort_by(|a, b| a.0.cmp(b.0));

    println!("Day-by-day breakdown:");
    for (day, (count, orig, filt)) in &days {
        let pct = if *orig > 0 {
            (*orig - *filt) as f64 / *orig as f64 * 100.0
        } else {
            0.0
        };
        let bar_len = (pct / 5.0) as usize;
        let bar = "█".repeat(bar_len.min(20));
        let empty = "░".repeat(20 - bar_len.min(20));
        println!(
            "  {}  {}{}  {:>4}cmds  {:>5}→{:>5} tokens ({:>3.0}%)",
            day, bar, empty, count, orig, filt, pct
        );
    }
}

fn print_daily_json(store: &AnalyticsStore) {
    let mut by_day: HashMap<String, (u64, u64, u64)> = HashMap::new();

    for entry in &store.entries {
        let day = entry.timestamp.format("%Y-%m-%d").to_string();
        let stats = by_day.entry(day).or_insert((0, 0, 0));
        stats.0 += 1;
        stats.1 += entry.original_tokens;
        stats.2 += entry.filtered_tokens;
    }

    let mut days: Vec<_> = by_day.iter().collect();
    days.sort_by(|a, b| a.0.cmp(b.0));

    println!("  \"daily\": [");
    for (i, (day, (count, orig, filt))) in days.iter().enumerate() {
        let pct = if *orig > 0 {
            (*orig - *filt) as f64 / *orig as f64 * 100.0
        } else {
            0.0
        };
        let comma = if i < days.len() - 1 { "," } else { "" };
        println!(
            "    {{\"date\": \"{}\", \"commands\": {}, \"original\": {}, \"filtered\": {}, \"savings_pct\": {:.1}}}{}",
            day, count, orig, filt, pct, comma
        );
    }
    println!("  ]");
}

fn print_history_json(store: &AnalyticsStore) {
    println!("  \"history\": [");
    for (i, entry) in store.entries.iter().rev().take(50).enumerate() {
        let comma = if i < 49.min(store.entries.len() - 1) {
            ","
        } else {
            ""
        };
        println!(
            "    {{\"timestamp\": \"{}\", \"command\": \"{}\", \"original\": {}, \"filtered\": {}}}{}",
            entry.timestamp,
            entry.command.escape_default(),
            entry.original_tokens,
            entry.filtered_tokens,
            comma
        );
    }
    println!("  ]");
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}
