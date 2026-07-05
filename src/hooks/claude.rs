use super::{merge_json_or_create, remove_hook_from_config};
use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude/settings.json")
    } else {
        PathBuf::from(".claude/settings.json")
    };

    let entry = serde_json::json!({
        "matcher": "Bash",
        "hooks": [{
            "type": "command",
            "command": "miskin hook claude"
        }]
    });

    merge_json_or_create(&path, "PreToolUse", &entry)?;
    println!("  Registered hook: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude/settings.json")
    } else {
        PathBuf::from(".claude/settings.json")
    };

    remove_hook_from_config(&path, "PreToolUse", "miskin hook claude")?;
    println!("  Removed hook from: {}", path.display());
    Ok(())
}
