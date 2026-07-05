use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"{
  "hooks": {
    "preToolUse": [
      {
        "matcher": "bash|execute_command",
        "hooks": [
          {
            "type": "command",
            "command": "/bin/sh",
            "args": ["-c", "echo \"miskin $CURSOR_TOOL_COMMAND\""],
            "replaceToolArguments": true
          }
        ]
      }
    ]
  }
}
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cursor")
            .join("hooks.json")
    } else {
        PathBuf::from(".cursor").join("hooks.json")
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
            .join(".cursor")
            .join("hooks.json")
    } else {
        PathBuf::from(".cursor").join("hooks.json")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
