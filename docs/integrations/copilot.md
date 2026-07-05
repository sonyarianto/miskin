# GitHub Copilot

## Install

```bash
miskin init --agent copilot          # Local
miskin init -g --agent copilot       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.github/hooks/miskin-rewrite.json` | Hook config — calls `miskin hook copilot` |
| `.github/copilot-instructions.md` | Caveman prompt |

Global install uses `~/.copilot/hooks/` and `~/.copilot/copilot-instructions.md`.

## How the hook works

A JSON config file with both `PreToolUse` (VS Code) and `preToolUse` (CLI) entries:

```json
{
  "version": 1,
  "hooks": {
    "PreToolUse": [{
      "type": "command",
      "command": "miskin hook copilot"
    }],
    "preToolUse": [{
      "type": "command",
      "bash": "miskin hook copilot",
      "powershell": "miskin hook copilot"
    }]
  }
}
```

`miskin hook copilot` reads stdin JSON, extracts the command, rewrites supported commands to `miskin <cmd>`, and returns the appropriate JSON for either VS Code or CLI format.

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `copilot-instructions.md`.

## Uninstall

```bash
miskin init --agent copilot --uninstall
```
