use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"// Miskin plugin for OpenCode
// Saves to .opencode/plugins/miskin.ts

export default {
  name: "miskin",
  description: "Save tokens by compressing command output through miskin",
  hooks: {
    "tool.execute.before": async (tool) => {
      const rewritable = ["bash", "shell", "execute"];

      if (rewritable.includes(tool.name)) {
        const cmd = typeof tool.input === "string"
          ? tool.input
          : tool.input?.command || tool.input?.cmd || "";

        const base = cmd.trim().split(/\s+/)[0];
        const supported = [
          "git", "ls", "cat", "find", "tree", "cargo", "npm",
          "pnpm", "yarn", "npx", "docker", "pytest", "jest",
          "vitest", "go", "eslint", "ruff", "biome", "clippy",
          "curl", "wget"
        ];

        if (supported.includes(base)) {
          if (typeof tool.input === "string") {
            tool.input = `miskin ${cmd}`;
          } else if (tool.input) {
            const key = tool.input.command ? "command" : "cmd";
            tool.input[key] = `miskin ${cmd}`;
          }
        }
      }

      return tool;
    }
  }
};
"#;

    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".opencode")
            .join("plugins")
            .join("miskin.ts")
    } else {
        PathBuf::from(".opencode").join("plugins").join("miskin.ts")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    println!("  Installed plugin: {}", path.display());
    Ok(())
}

pub fn uninstall(global: bool) -> anyhow::Result<()> {
    let path = if global {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".opencode")
            .join("plugins")
            .join("miskin.ts")
    } else {
        PathBuf::from(".opencode").join("plugins").join("miskin.ts")
    };
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("  Removed: {}", path.display());
    }
    Ok(())
}
