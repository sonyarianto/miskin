# Gemini CLI

## Install

```bash
miskin init --agent gemini          # Local
miskin init -g --agent gemini       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.gemini/settings.json` | BeforeTool hook — calls wrapper script |
| `.gemini/hooks/miskin-hook-gemini.sh` | Wrapper script that execs `miskin hook gemini` |
| `GEMINI.md` | Caveman prompt |

## How the hook works

A BeforeTool hook is registered in Gemini's `settings.json`:

```json
{
  "hooks": {
    "BeforeTool": [{
      "matcher": "run_shell_command",
      "hooks": [{
        "type": "command",
        "command": "/home/user/.gemini/hooks/miskin-hook-gemini.sh"
      }]
    }]
  }
}
```

The wrapper script simply execs the miskin binary:

```bash
#!/bin/bash
exec miskin hook gemini
```

`miskin hook gemini` reads stdin JSON, extracts the command, and returns Gemini's protocol:

```json
{
  "decision": "allow",
  "hookSpecificOutput": {
    "tool_input": { "command": "miskin docker ps" }
  }
}
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `GEMINI.md`.

## Uninstall

```bash
miskin init --agent gemini --uninstall
```
