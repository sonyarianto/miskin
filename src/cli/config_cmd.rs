use crate::config::MiskinConfig;

pub fn run() -> anyhow::Result<()> {
    let config = MiskinConfig::load()?;
    let toml = toml::to_string_pretty(&config)?;
    println!("{}", toml);
    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    let config = MiskinConfig::default();
    config.save()?;
    println!("Configuration reset to defaults.");
    Ok(())
}

pub fn set(key: &str, value: &str) -> anyhow::Result<()> {
    let mut config = MiskinConfig::load()?;

    match key {
        "caveman.enabled" => {
            config.caveman.enabled = value.parse()?;
        }
        "caveman.level" => {
            config.caveman.level = serde_json::from_str(&format!("\"{}\"", value))?;
        }
        "general.enabled" => {
            config.general.enabled = value.parse()?;
        }
        "general.ultra_compact" => {
            config.general.ultra_compact = value.parse()?;
        }
        "analytics.enabled" => {
            config.analytics.enabled = value.parse()?;
        }
        "filters.deduplicate" => {
            config.filters.deduplicate = value.parse()?;
        }
        "filters.strip_comments" => {
            config.filters.strip_comments = value.parse()?;
        }
        "filters.max_lines" => {
            config.filters.max_lines = value.parse()?;
        }
        _ => {
            anyhow::bail!(
                "Unknown config key '{}'. Available: caveman.enabled, caveman.level, general.enabled, general.ultra_compact, analytics.enabled, filters.deduplicate, filters.strip_comments, filters.max_lines",
                key
            );
        }
    }

    config.save()?;
    println!("Set {} = {}", key, value);
    Ok(())
}
