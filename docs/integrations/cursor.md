# Cursor

## Install

```bash
miskin init --agent cursor          # Local
miskin init -g --agent cursor       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.cursor/hooks.json` | preToolUse hook — calls `miskin hook cursor` |
| `.cursorrules` | Caveman prompt |

## How the hook works

Miskin registers in `hooks.json` under `preToolUse`:

```json
{
  "hooks": {
    "preToolUse": [{
      "matcher": "Shell",
      "command": "miskin hook cursor"
    }]
  }
}
```

`miskin hook cursor` reads stdin JSON, extracts the shell command, rewrites supported commands, and returns Cursor's protocol format:

```json
{
  "continue": true,
  "permission": "allow",
  "updated_input": { "command": "miskin git status" }
}
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `.cursorrules`.

## Uninstall

```bash
miskin init --agent cursor --uninstall
```
