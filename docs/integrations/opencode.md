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

OpenCode auto-loads TypeScript plugins from `.opencode/plugins/`. The plugin intercepts `tool.execute.before` and rewrites bash commands:

```ts
export const MiskinPlugin = async (ctx) => {
  return {
    "tool.execute.before": async (input, output) => {
      if (input.tool === "bash") {
        const cmd = output.args.command || "";
        const base = cmd.trim().split(/\s+/)[0];
        if (SUPPORTED.includes(base)) {
          output.args.command = `miskin ${cmd}`;
        }
      }
    },
  };
};
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `OPENCODE.md`.

## Uninstall

```bash
miskin init --agent opencode --uninstall
```
