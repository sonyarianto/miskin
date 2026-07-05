use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"# Miskin for Codex
# Reads from .codex/miskin-instructions.md

**MISKIN ACTIVE** — Compress all command outputs through miskin.

When executing shell commands, always prefix with `miskin`:
- `miskin git status` instead of `git status`
- `miskin cargo test` instead of `cargo test`
- `miskin ls` instead of `ls`

Supported commands: git, cargo, npm, pnpm, yarn, docker, ls, cat, find, tree,
pytest, jest, vitest, go, eslint, ruff, biome, clippy, curl, wget
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".codex")
            .join("miskin.md")
    } else {
        PathBuf::from(".codex").join("miskin.md")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    println!("  Installed instructions: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".codex")
            .join("miskin.md")
    } else {
        PathBuf::from(".codex").join("miskin.md")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
