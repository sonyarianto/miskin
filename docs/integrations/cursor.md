# Cursor

## Install

```bash
miskin init --agent cursor          # Local
miskin init -g --agent cursor       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.cursor/hooks.json` | preToolUse hook configuration |
| `.cursorrules` | Caveman prompt |

## How the hook works

Cursor's `hooks.json` supports a `preToolUse` hook that rewrites bash commands:

```json
{
  "hooks": {
    "preToolUse": [
      {
        "matcher": "bash|execute_command",
        "hooks": [
          {
            "type": "command",
            "command": "/bin/sh",
            "args": ["-c", "echo \"miskin $CURSOR_TOOL_COMMAND\""],
            "replaceToolArguments": true
          }
        ]
      }
    ]
  }
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
