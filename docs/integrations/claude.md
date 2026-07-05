# Claude Code

## Install

```bash
miskin init          # Local (project)
miskin init -g       # Global (all projects)
```

## What gets installed

| File | Purpose |
|------|---------|
| `.claude/hooks/miskin-rewrite.sh` | PreToolUse hook — rewrites `git status` → `miskin git status` |
| `CLAUDE.md` (or `~/.claude/MISKIN.md`) | Caveman prompt injection |

## How the hook works

The hook script intercepts bash tool calls and rewrites supported commands:

```bash
#!/usr/bin/env bash
# When Claude runs: git status
# Hook rewrites to: miskin git status

case "$base" in
    git|ls|cat|find|tree|cargo|npm|pnpm|yarn|npx|docker|...)
        echo "miskin $cmd"
        ;;
esac
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
miskin config set caveman.level ultra
```

This appends caveman instructions to `CLAUDE.md`, which Claude reads at session start.

## Uninstall

```bash
miskin init --uninstall
```

Removes `.claude/hooks/miskin-rewrite.sh` and the caveman block from `CLAUDE.md`.
