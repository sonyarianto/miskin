use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".copilot/hooks/miskin-rewrite.json")
    } else {
        PathBuf::from(".github/hooks/miskin-rewrite.json")
    };

    let config = serde_json::json!({
        "version": 1,
        "hooks": {
            "PreToolUse": [{
                "type": "command",
                "command": "miskin hook copilot",
                "cwd": ".",
                "timeout": 5
            }],
            "preToolUse": [{
                "type": "command",
                "bash": "miskin hook copilot",
                "powershell": "miskin hook copilot",
                "cwd": ".",
                "timeoutSec": 5
            }]
        }
    });

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(&config)?)?;
    println!("  Installed hook: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".copilot/hooks/miskin-rewrite.json")
    } else {
        PathBuf::from(".github/hooks/miskin-rewrite.json")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
