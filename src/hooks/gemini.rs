use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"# Miskin - Gemini CLI hook
# Place in ~/.gemini/settings.json or project .gemini/settings.json

{
  "hooks": {
    "beforeTool": {
      "bash": {
        "rewrite": "miskin {command}"
      }
    }
  }
}
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".gemini")
            .join("miskin-hook.json")
    } else {
        PathBuf::from(".gemini").join("miskin-hook.json")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    println!("  Installed hook: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".gemini")
            .join("miskin-hook.json")
    } else {
        PathBuf::from(".gemini").join("miskin-hook.json")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
