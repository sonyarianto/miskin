# Windsurf

## Install

```bash
miskin init --agent windsurf          # Local
miskin init -g --agent windsurf       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.windsurfrules` | Rules file — instructs Windsurf to use `miskin` |
| `.windsurfrules` | Also serves as caveman prompt when enabled |

## How it works

Windsurf reads `.windsurfrules` as project-scoped instructions. The file tells it to prefix supported commands with `miskin`.

```markdown
**MISKIN ACTIVE**

When executing shell commands, prefix supported commands with `miskin`:
git status → miskin git status
cargo test → miskin cargo test
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

Writes to `.windsurfrules`.

## Uninstall

```bash
miskin init --agent windsurf --uninstall
```
