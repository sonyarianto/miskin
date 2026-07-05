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

The TypeScript plugin intercepts tool executions:

```ts
export default {
  name: "miskin",
  hooks: {
    "tool.execute.before": async (tool) => {
      if (["bash", "shell", "execute"].includes(tool.name)) {
        const cmd = tool.input?.command || tool.input?.cmd || "";
        const base = cmd.trim().split(/\s+/)[0];
        if (["git", "cargo", "npm", ...].includes(base)) {
          tool.input.command = `miskin ${cmd}`;
        }
      }
      return tool;
    }
  }
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
