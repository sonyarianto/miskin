use std::path::PathBuf;

pub fn install(global: bool) -> anyhow::Result<()> {
    let content = r#"// Miskin plugin for OpenCode
// Auto-loaded from .opencode/plugins/

const SUPPORTED = [
  "git", "ls", "cat", "find", "tree", "cargo", "npm",
  "pnpm", "yarn", "npx", "docker", "pytest", "jest",
  "vitest", "go", "eslint", "ruff", "biome", "clippy",
  "curl", "wget", "df", "du", "ps", "wc", "env",
  "gh", "kubectl", "oc",
];

export const MiskinPlugin = async (ctx) => {
  return {
    "tool.execute.before": async (input, output) => {
      if (input.tool !== "bash" && input.tool !== "shell" && input.tool !== "execute") {
        return;
      }

      const cmd = typeof output.args === "string"
        ? output.args
        : output.args?.command || output.args?.cmd || "";

      const base = cmd.trim().split(/\s+/)[0];
      if (!base || !SUPPORTED.includes(base)) {
        return;
      }

      if (typeof output.args === "string") {
        output.args = `miskin ${cmd}`;
      } else if (output.args?.command) {
        output.args.command = `miskin ${cmd}`;
      } else if (output.args?.cmd) {
        output.args.cmd = `miskin ${cmd}`;
      }
    },
  };
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
