pub mod counter;
pub mod report;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub command: String,
    pub original_tokens: u64,
    pub filtered_tokens: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalyticsStore {
    pub entries: Vec<SessionEntry>,
    pub total_original: u64,
    pub total_filtered: u64,
    pub command_count: u64,
}

impl AnalyticsStore {
    pub fn load(data_dir: &std::path::Path) -> anyhow::Result<Self> {
        let path = data_dir.join("analytics.json");
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            if content.trim().is_empty() {
                return Ok(Self::default());
            }
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, data_dir: &std::path::Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join("analytics.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn record(&mut self, command: &str, original: u64, filtered: u64) {
        self.entries.push(SessionEntry {
            command: command.to_string(),
            original_tokens: original,
            filtered_tokens: filtered,
            timestamp: chrono::Utc::now(),
        });
        self.total_original += original;
        self.total_filtered += filtered;
        self.command_count += 1;

        if self.entries.len() > 10000 {
            self.entries.drain(0..1000);
        }
    }

    pub fn savings_pct(&self) -> f64 {
        if self.total_original == 0 {
            return 0.0;
        }
        ((self.total_original - self.total_filtered) as f64 / self.total_original as f64) * 100.0
    }

    pub fn total_saved(&self) -> u64 {
        self.total_original.saturating_sub(self.total_filtered)
    }
}
