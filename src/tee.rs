use std::path::PathBuf;

pub fn save_raw(
    data_dir: &std::path::Path,
    command: &str,
    output: &str,
) -> anyhow::Result<PathBuf> {
    let tee_dir = data_dir.join("tee");
    std::fs::create_dir_all(&tee_dir)?;

    let now = chrono::Utc::now();
    let timestamp = format!(
        "{}{:03}",
        now.format("%Y%m%d_%H%M%S"),
        now.timestamp_subsec_millis()
    );
    let safe_name = command
        .replace([' ', '/', '\\'], "_")
        .chars()
        .take(60)
        .collect::<String>();

    let filename = format!("{}_{}.log", timestamp, safe_name);
    let path = tee_dir.join(&filename);

    std::fs::write(&path, output)?;

    Ok(path)
}
