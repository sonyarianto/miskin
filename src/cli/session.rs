use crate::analytics::AnalyticsStore;
use crate::config::MiskinConfig;
use std::collections::HashMap;

pub fn run(json: bool) -> anyhow::Result<()> {
    let config = MiskinConfig::load()?;
    let store = AnalyticsStore::load(&config.analytics.data_dir).unwrap_or_default();

    if store.entries.is_empty() {
        if json {
            println!("{{\"sessions\": [], \"total_commands\": 0}}");
        } else {
            println!("No session data yet.");
        }
        return Ok(());
    }

    let mut sessions: Vec<Vec<&crate::analytics::SessionEntry>> = Vec::new();
    let mut current: Vec<&crate::analytics::SessionEntry> = Vec::new();
    let gap_minutes = 30;

    for entry in &store.entries {
        if let Some(last) = current.last() {
            let diff = (entry.timestamp - last.timestamp).num_minutes();
            if diff > gap_minutes && !current.is_empty() {
                sessions.push(std::mem::take(&mut current));
            }
        }
        current.push(entry);
    }
    if !current.is_empty() {
        sessions.push(current);
    }

    if json {
        println!("{{");
        println!("  \"session_count\": {},", sessions.len());
        println!("  \"total_commands\": {},", store.entries.len());
        println!("  \"sessions\": [");
        for (i, session) in sessions.iter().enumerate() {
            let orig: u64 = session.iter().map(|e| e.original_tokens).sum();
            let filt: u64 = session.iter().map(|e| e.filtered_tokens).sum();
            println!("    {{");
            println!(
                "      \"start\": \"{}\",",
                session.first().unwrap().timestamp
            );
            println!("      \"end\": \"{}\",", session.last().unwrap().timestamp);
            println!("      \"commands\": {},", session.len());
            println!("      \"original_tokens\": {},", orig);
            println!("      \"filtered_tokens\": {},", filt);
            println!(
                "      \"savings_pct\": {:.1}",
                if orig > 0 {
                    (orig - filt) as f64 / orig as f64 * 100.0
                } else {
                    0.0
                }
            );
            if i < sessions.len() - 1 {
                println!("    }},");
            } else {
                println!("    }}");
            }
        }
        println!("  ]");
        println!("}}");
    } else {
        println!(
            "Miskin Adoption — {} sessions, {} total commands\n",
            sessions.len(),
            store.entries.len()
        );

        let mut by_cmd: HashMap<String, u64> = HashMap::new();
        for entry in &store.entries {
            let cmd = entry
                .command
                .split_whitespace()
                .next()
                .unwrap_or(&entry.command)
                .to_string();
            *by_cmd.entry(cmd).or_insert(0) += 1;
        }
        let mut sorted: Vec<_> = by_cmd.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        println!("Top commands:");
        for (cmd, count) in sorted.iter().take(10) {
            let bar = "█".repeat((**count as usize).min(50));
            println!("  {:<15} {:>4} {}", cmd, count, bar);
        }

        for (i, session) in sessions.iter().enumerate() {
            let orig: u64 = session.iter().map(|e| e.original_tokens).sum();
            let filt: u64 = session.iter().map(|e| e.filtered_tokens).sum();
            let pct = if orig > 0 {
                (orig - filt) as f64 / orig as f64 * 100.0
            } else {
                0.0
            };

            if let (Some(first), Some(last)) = (session.first(), session.last()) {
                println!(
                    "  Session {}: {} → {}  {}cmds  {}→{} tokens ({:.0}%)",
                    i + 1,
                    first.timestamp.format("%H:%M"),
                    last.timestamp.format("%H:%M"),
                    session.len(),
                    orig,
                    filt,
                    pct,
                );
            }
        }
    }

    Ok(())
}
