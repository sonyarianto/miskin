use crate::analytics::AnalyticsStore;
use crate::config::MiskinConfig;
use crate::filters::FilterRegistry;
use std::collections::HashMap;

pub fn run(all: bool, since: u32) -> anyhow::Result<()> {
    let config = MiskinConfig::load()?;
    let store = AnalyticsStore::load(&config.analytics.data_dir).unwrap_or_default();
    let registry = FilterRegistry::default();

    let cutoff = chrono::Utc::now() - chrono::Duration::days(since as i64);

    let mut by_command: HashMap<String, (u64, u64, u64)> = HashMap::new();

    for entry in &store.entries {
        if !all && entry.timestamp < cutoff {
            continue;
        }
        let cmd = entry.command.split_whitespace().next().unwrap_or(&entry.command).to_string();
        let stats = by_command.entry(cmd).or_insert((0, 0, 0));
        stats.0 += 1;
        stats.1 += entry.original_tokens;
        stats.2 += entry.filtered_tokens;
    }

    let mut missed: Vec<(&String, &(u64, u64, u64))> = by_command
        .iter()
        .filter(|(cmd, _)| !registry.has(cmd))
        .collect();

    missed.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    if missed.is_empty() {
        println!("No missed opportunities! All commands are filtered.");
        return Ok(());
    }

    println!("Commands with no filter (potential savings):");
    for (cmd, (count, orig, filt)) in missed.iter().take(20) {
        println!(
            "  {:<20} {} calls, {} tokens (possible filter could save ~{} tokens)",
            cmd,
            count,
            orig,
            orig.saturating_sub(*filt)
        );
    }
    println!();

    let total_unfiltered: u64 = missed.iter().map(|(_, (_, o, _))| o).sum();
    println!(
        "Total: {} unfiltered commands, {} potential tokens to save.",
        missed.len(),
        total_unfiltered / 2
    );

    Ok(())
}
