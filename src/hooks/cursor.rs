use super::{merge_json_or_create, remove_hook_from_config};
use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cursor/hooks.json")
    } else {
        PathBuf::from(".cursor/hooks.json")
    };

    let entry = serde_json::json!({
        "matcher": "Shell",
        "command": "miskin hook cursor"
    });

    merge_json_or_create(&path, "preToolUse", &entry)?;
    println!("  Registered hook: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cursor/hooks.json")
    } else {
        PathBuf::from(".cursor/hooks.json")
    };

    remove_hook_from_config(&path, "preToolUse", "miskin hook cursor")?;
    println!("  Removed hook from: {}", path.display());
    Ok(())
}
