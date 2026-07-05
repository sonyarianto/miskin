# Claude Code

## Install

```bash
miskin init          # Local (project)
miskin init -g       # Global (all projects)
```

## What gets installed

| File | Purpose |
|------|---------|
| `.claude/settings.json` | PreToolUse hook — calls `miskin hook claude` |
| `CLAUDE.md` | Caveman prompt injection |

## How the hook works

Miskin registers a `PreToolUse` hook in Claude Code's `settings.json`. When Claude runs a Bash command, the hook spawns `miskin hook claude` which reads stdin JSON, checks if the command is supported, and returns JSON with the rewritten command:

`settings.json` entry:
```json
{
  "hooks": {
    "PreToolUse": [{
      "matcher": "Bash",
      "hooks": [{
        "type": "command",
        "command": "miskin hook claude"
      }]
    }]
  }
}
```

`git status` → rewritten to `miskin git status` via JSON response. Unrecognized commands pass through unchanged.

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
miskin config set caveman.level ultra
```

Appends caveman instructions to `CLAUDE.md`.

## Uninstall

```bash
miskin init --uninstall
miskin init -g --uninstall
```

Removes the miskin entry from `settings.json` and the caveman block from `CLAUDE.md`.
