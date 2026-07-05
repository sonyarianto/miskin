use super::{merge_json_or_create, remove_hook_from_config};
use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let settings_path = if global {
        home.join(".gemini/settings.json")
    } else {
        PathBuf::from(".gemini/settings.json")
    };
    let wrapper_path = if global {
        home.join(".gemini/hooks/miskin-hook-gemini.sh")
    } else {
        PathBuf::from(".gemini/hooks/miskin-hook-gemini.sh")
    };

    let wrapper = "#!/bin/bash\nexec miskin hook gemini\n";

    if let Some(parent) = wrapper_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&wrapper_path, wrapper)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&wrapper_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&wrapper_path, perms)?;
    }

    let entry = serde_json::json!({
        "matcher": "run_shell_command",
        "hooks": [{
            "type": "command",
            "command": wrapper_path.to_string_lossy().to_string()
        }]
    });

    merge_json_or_create(&settings_path, "BeforeTool", &entry)?;
    println!("  Installed wrapper: {}", wrapper_path.display());
    println!("  Registered hook: {}", settings_path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let settings_path = if global {
        home.join(".gemini/settings.json")
    } else {
        PathBuf::from(".gemini/settings.json")
    };
    let wrapper_path = if global {
        home.join(".gemini/hooks/miskin-hook-gemini.sh")
    } else {
        PathBuf::from(".gemini/hooks/miskin-hook-gemini.sh")
    };

    remove_hook_from_config(&settings_path, "BeforeTool", "miskin hook gemini")?;
    if wrapper_path.exists() {
        std::fs::remove_file(&wrapper_path)?;
    }
    println!("  Removed hook from: {}", settings_path.display());
    Ok(())
}
