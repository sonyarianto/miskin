# Gemini CLI

## Install

```bash
miskin init --agent gemini          # Local
miskin init -g --agent gemini       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.gemini/miskin-hook.json` | BeforeTool hook configuration |
| `GEMINI.md` | Caveman prompt |

## How the hook works

Gemini CLI supports `beforeTool` hooks:

```json
{
  "hooks": {
    "beforeTool": {
      "bash": {
        "rewrite": "miskin {command}"
      }
    }
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
