use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"// Miskin hook for GitHub Copilot
// Place in .github/copilot-hooks/ or configure via VS Code settings

module.exports = {
  preToolUse: async (toolCall) => {
    const rewritable = ['git', 'ls', 'cat', 'find', 'cargo', 'npm', 'pnpm',
      'yarn', 'npx', 'docker', 'pytest', 'jest', 'vitest', 'go',
      'eslint', 'ruff', 'curl', 'wget'];
    if (toolCall.name === 'execute_command' || toolCall.name === 'bash') {
      const cmd = toolCall.arguments?.command || '';
      const base = cmd.split(' ')[0];
      if (rewritable.includes(base)) {
        toolCall.arguments.command = `miskin ${cmd}`;
      }
    }
    return toolCall;
  }
};
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".github")
            .join("copilot-hooks")
            .join("miskin-hook.js")
    } else {
        PathBuf::from(".github")
            .join("copilot-hooks")
            .join("miskin-hook.js")
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
            .join(".github")
            .join("copilot-hooks")
            .join("miskin-hook.js")
    } else {
        PathBuf::from(".github")
            .join("copilot-hooks")
            .join("miskin-hook.js")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
