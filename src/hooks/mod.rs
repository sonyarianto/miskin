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
            home.join(".claude/hooks/miskin-rewrite.sh")
        } else {
            PathBuf::from(".claude/hooks/miskin-rewrite.sh")
        }],
        "copilot" => vec![if global {
            home.join(".github/copilot-hooks/miskin-hook.js")
        } else {
            PathBuf::from(".github/copilot-hooks/miskin-hook.js")
        }],
        "cursor" => vec![if global {
            home.join(".cursor/hooks.json")
        } else {
            PathBuf::from(".cursor/hooks.json")
        }],
        "gemini" => vec![if global {
            home.join(".gemini/miskin-hook.json")
        } else {
            PathBuf::from(".gemini/miskin-hook.json")
        }],
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
            home.join(".github/MISKIN.md")
        } else {
            PathBuf::from(".github/MISKIN.md")
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
