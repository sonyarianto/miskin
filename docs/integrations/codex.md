# Codex

## Install

```bash
miskin init --agent codex          # Local
miskin init -g --agent codex       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.codex/miskin.md` | Instructions file — tells Codex to prefix commands with `miskin` |
| `AGENTS.md` | Caveman prompt |

## How it works

Codex reads `.codex/miskin.md` at session start. The instructions tell it to wrap supported commands:

```markdown
**MISKIN ACTIVE** — Compress all command outputs through miskin.

When executing shell commands, always prefix with `miskin`:
- `miskin git status` instead of `git status`
- `miskin cargo test` instead of `cargo test`
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `AGENTS.md`.

## Uninstall

```bash
miskin init --agent codex --uninstall
```
