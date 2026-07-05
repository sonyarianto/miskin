use std::io::Read;

const REWRITABLE: &[&str] = &[
    "git", "ls", "cat", "find", "tree", "cargo", "npm", "pnpm", "yarn", "npx", "docker", "pytest",
    "jest", "vitest", "go", "eslint", "ruff", "biome", "clippy", "curl", "wget", "df", "du", "ps",
    "wc", "env", "gh", "kubectl", "oc",
];

fn should_rewrite(cmd: &str) -> bool {
    let base = cmd.split_whitespace().next().unwrap_or("");
    REWRITABLE.contains(&base)
}

fn extract_command(input: &serde_json::Value) -> Option<String> {
    input
        .get("tool_input")
        .or_else(|| input.get("toolArgs"))
        .and_then(|v| {
            if let Some(cmd) = v.get("command") {
                Some(cmd.as_str()?.to_string())
            } else if let Some(s) = v.as_str() {
                serde_json::from_str::<serde_json::Value>(s)
                    .ok()
                    .and_then(|parsed| parsed.get("command")?.as_str().map(String::from))
            } else {
                None
            }
        })
}

fn rewrite_command(cmd: &str) -> String {
    format!("miskin {}", cmd)
}

// ─── Claude Code ──────────────────────────────────────────────────

fn hook_claude(input: &serde_json::Value) -> serde_json::Value {
    let cmd = match extract_command(input) {
        Some(c) => c,
        None => return serde_json::json!({}),
    };

    if !should_rewrite(&cmd) {
        return serde_json::json!({});
    }

    let rewritten = rewrite_command(&cmd);
    serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "allow",
            "permissionDecisionReason": "Miskin auto-rewrite",
            "updatedInput": { "command": rewritten }
        }
    })
}

// ─── Copilot ───────────────────────────────────────────────────────

fn hook_copilot(input: &serde_json::Value) -> serde_json::Value {
    let cmd = match extract_command(input) {
        Some(c) => c,
        None => return serde_json::json!({}),
    };

    if !should_rewrite(&cmd) {
        return serde_json::json!({});
    }

    let rewritten = rewrite_command(&cmd);

    if input.get("toolName").is_some() {
        serde_json::json!({
            "permissionDecision": "allow",
            "permissionDecisionReason": "Miskin auto-rewrite",
            "modifiedArgs": { "command": rewritten }
        })
    } else {
        serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "Miskin auto-rewrite",
                "updatedInput": { "command": rewritten }
            }
        })
    }
}

// ─── Cursor ────────────────────────────────────────────────────────

fn hook_cursor(input: &serde_json::Value) -> serde_json::Value {
    let cmd = match extract_command(input) {
        Some(c) => c,
        None => return serde_json::json!({}),
    };

    if !should_rewrite(&cmd) {
        return serde_json::json!({});
    }

    let rewritten = rewrite_command(&cmd);
    serde_json::json!({
        "continue": true,
        "permission": "allow",
        "updated_input": { "command": rewritten }
    })
}

// ─── Gemini ────────────────────────────────────────────────────────

fn hook_gemini(input: &serde_json::Value) -> serde_json::Value {
    let cmd = match extract_command(input) {
        Some(c) => c,
        None => return serde_json::json!({"decision": "allow"}),
    };

    if !should_rewrite(&cmd) {
        return serde_json::json!({"decision": "allow"});
    }

    let rewritten = rewrite_command(&cmd);
    serde_json::json!({
        "decision": "allow",
        "hookSpecificOutput": { "tool_input": { "command": rewritten } }
    })
}

// ─── Dispatcher ────────────────────────────────────────────────────

pub fn run(agent: &str) -> anyhow::Result<()> {
    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin)?;

    let input: serde_json::Value = if stdin.trim().is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_str(&stdin).unwrap_or(serde_json::Value::Null)
    };

    let output = match agent {
        "claude" => hook_claude(&input),
        "copilot" => hook_copilot(&input),
        "cursor" => hook_cursor(&input),
        "gemini" => hook_gemini(&input),
        _ => serde_json::json!({}),
    };

    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_rewrites_git() {
        let input = serde_json::json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git status" }
        });
        let out = hook_claude(&input);
        assert_eq!(
            out["hookSpecificOutput"]["updatedInput"]["command"],
            "miskin git status"
        );
        assert_eq!(out["hookSpecificOutput"]["permissionDecision"], "allow");
    }

    #[test]
    fn claude_passthrough_unknown() {
        let input = serde_json::json!({
            "tool_name": "Bash",
            "tool_input": { "command": "echo hello" }
        });
        let out = hook_claude(&input);
        assert_eq!(out, serde_json::json!({}));
    }

    #[test]
    fn copilot_rewrites_cargo() {
        let input = serde_json::json!({
            "tool_name": "runTerminalCommand",
            "tool_input": { "command": "cargo test" }
        });
        let out = hook_copilot(&input);
        assert_eq!(
            out["hookSpecificOutput"]["updatedInput"]["command"],
            "miskin cargo test"
        );
    }

    #[test]
    fn copilot_cli_format() {
        let input = serde_json::json!({
            "toolName": "bash",
            "toolArgs": "{\"command\": \"npm install\"}"
        });
        let out = hook_copilot(&input);
        assert_eq!(out["modifiedArgs"]["command"], "miskin npm install");
    }

    #[test]
    fn cursor_rewrites_ls() {
        let input = serde_json::json!({
            "tool_name": "Bash",
            "tool_input": { "command": "ls -la" }
        });
        let out = hook_cursor(&input);
        assert_eq!(out["updated_input"]["command"], "miskin ls -la");
        assert_eq!(out["permission"], "allow");
    }

    #[test]
    fn cursor_passthrough() {
        let input = serde_json::json!({
            "tool_name": "Bash",
            "tool_input": { "command": "make install" }
        });
        let out = hook_cursor(&input);
        assert_eq!(out, serde_json::json!({}));
    }

    #[test]
    fn gemini_rewrites_docker() {
        let input = serde_json::json!({
            "tool_name": "run_shell_command",
            "tool_input": { "command": "docker ps" }
        });
        let out = hook_gemini(&input);
        assert_eq!(out["decision"], "allow");
        assert_eq!(
            out["hookSpecificOutput"]["tool_input"]["command"],
            "miskin docker ps"
        );
    }
}
