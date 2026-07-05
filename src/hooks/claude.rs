use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let hook_script = r#"#!/usr/bin/env bash
# Miskin hook for Claude Code
# Rewrites commands: git status -> miskin git status

rewrite_command() {
    local cmd="$1"
    local base="${cmd%% *}"
    local rest="${cmd#* }"
    [[ "$rest" == "$cmd" ]] && rest=""

    case "$base" in
        git|ls|cat|find|tree|cargo|npm|pnpm|yarn|npx|docker|pytest|jest|vitest|go|eslint|ruff|biome|clippy|curl|wget)
            echo "miskin $cmd"
            return 0
            ;;
        *)
            echo "$cmd"
            return 0
            ;;
    esac
}

# When used as a hook, read the command from stdin
if [ -n "$1" ]; then
    rewrite_command "$*"
else
    while IFS= read -r line; do
        rewrite_command "$line"
    done
fi
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude")
            .join("hooks")
            .join("miskin-rewrite.sh")
    } else {
        PathBuf::from(".claude").join("hooks").join("miskin-rewrite.sh")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, hook_script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms)?;
    }

    println!("  Installed hook: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude")
            .join("hooks")
            .join("miskin-rewrite.sh")
    } else {
        PathBuf::from(".claude").join("hooks").join("miskin-rewrite.sh")
    };

    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
