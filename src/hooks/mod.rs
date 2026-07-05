pub mod claude;
pub mod codex;
pub mod copilot;
pub mod cursor;
pub mod gemini;
pub mod opencode;
pub mod windsurf;

use std::path::PathBuf;

pub fn paths_for_agent(agent: &str, global: bool) -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    match agent {
        "claude" => vec![if global {
            home.join(".claude/settings.json")
        } else {
            PathBuf::from(".claude/settings.json")
        }],
        "copilot" => vec![if global {
            home.join(".copilot/hooks/miskin-rewrite.json")
        } else {
            PathBuf::from(".github/hooks/miskin-rewrite.json")
        }],
        "cursor" => vec![if global {
            home.join(".cursor/hooks.json")
        } else {
            PathBuf::from(".cursor/hooks.json")
        }],
        "gemini" => vec![
            if global {
                home.join(".gemini/settings.json")
            } else {
                PathBuf::from(".gemini/settings.json")
            },
            if global {
                home.join(".gemini/hooks/miskin-hook-gemini.sh")
            } else {
                PathBuf::from(".gemini/hooks/miskin-hook-gemini.sh")
            },
        ],
        "codex" => vec![if global {
            home.join(".codex/miskin.md")
        } else {
            PathBuf::from(".codex/miskin.md")
        }],
        "opencode" => vec![if global {
            home.join(".opencode/plugins/miskin.ts")
        } else {
            PathBuf::from(".opencode/plugins/miskin.ts")
        }],
        "windsurf" => vec![if global {
            home.join(".windsurfrules")
        } else {
            PathBuf::from(".windsurfrules")
        }],
        _ => vec![],
    }
}

pub fn prompt_paths_for_agent(agent: &str, global: bool) -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    match agent {
        "claude" => vec![if global {
            home.join(".claude/MISKIN.md")
        } else {
            PathBuf::from("CLAUDE.md")
        }],
        "copilot" => vec![if global {
            home.join(".copilot/copilot-instructions.md")
        } else {
            PathBuf::from(".github/copilot-instructions.md")
        }],
        "cursor" => vec![if global {
            home.join(".cursorrules")
        } else {
            PathBuf::from(".cursorrules")
        }],
        "gemini" => vec![if global {
            home.join(".gemini/GEMINI.md")
        } else {
            PathBuf::from("GEMINI.md")
        }],
        "codex" => vec![if global {
            home.join(".codex/AGENTS.md")
        } else {
            PathBuf::from("AGENTS.md")
        }],
        "opencode" => vec![if global {
            home.join(".opencode/OPENCODE.md")
        } else {
            PathBuf::from("OPENCODE.md")
        }],
        "windsurf" => vec![if global {
            home.join(".windsurfrules")
        } else {
            PathBuf::from(".windsurfrules")
        }],
        _ => vec![],
    }
}

fn merge_json_or_create(
    path: &PathBuf,
    hook_key: &str,
    new_entry: &serde_json::Value,
) -> anyhow::Result<()> {
    let mut config: serde_json::Value = if path.exists() {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let hooks = config.as_object_mut().and_then(|c| {
        c.entry("hooks")
            .or_insert_with(|| serde_json::json!({}))
            .as_object_mut()
    });

    if let Some(hooks_obj) = hooks {
        let arr = hooks_obj
            .entry(hook_key)
            .or_insert_with(|| serde_json::json!([]))
            .as_array_mut();

        if let Some(arr) = arr
            && !arr.contains(new_entry)
        {
            arr.push(new_entry.clone());
        }
    }

    let content = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, content)?;
    Ok(())
}

fn remove_hook_from_config(
    path: &PathBuf,
    hook_key: &str,
    command_pattern: &str,
) -> anyhow::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let content = std::fs::read_to_string(path)?;
    let mut config: serde_json::Value =
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}));

    if let Some(arr) = config
        .get_mut("hooks")
        .and_then(|h| h.get_mut(hook_key))
        .and_then(|v| v.as_array_mut())
    {
        arr.retain(|entry| {
            let s = serde_json::to_string(entry).unwrap_or_default();
            !s.contains(command_pattern)
        });
    }

    std::fs::write(path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}
