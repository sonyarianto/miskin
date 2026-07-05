# OpenCode

## Install

```bash
miskin init --agent opencode          # Local
miskin init -g --agent opencode       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.opencode/plugins/miskin.ts` | TypeScript plugin — `tool.execute.before` hook |
| `OPENCODE.md` | Caveman prompt |

## How the hook works

OpenCode auto-loads TypeScript plugins from `.opencode/plugins/`. The plugin intercepts `tool.execute.before` and rewrites bash/shell/execute commands:

```ts
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
      if (!base || !SUPPORTED.includes(base)) return;
      output.args.command = `miskin ${cmd}`;
    },
  };
};
```

Supports 28 command types: git, cargo, npm, docker, pytest, eslint, gh, kubectl, and more.

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `OPENCODE.md`.

## Uninstall

```bash
miskin init --agent opencode --uninstall
```
