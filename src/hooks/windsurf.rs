use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"// Miskin for Windsurf
// .windsurfrules

**MISKIN ACTIVE**

When executing shell commands, prefix supported commands with `miskin`:
git status → miskin git status
cargo test → miskin cargo test
ls → miskin ls
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".windsurfrules")
    } else {
        PathBuf::from(".windsurfrules")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    println!("  Installed rules: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".windsurfrules")
    } else {
        PathBuf::from(".windsurfrules")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
