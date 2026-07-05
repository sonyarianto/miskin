# How It Works

Miskin uses two strategies to save tokens:

## 1. Command Output Filtering

When your AI tool runs a shell command (via hooks), Miskin intercepts it:

```
AI tool:  git status
Hook:     miskin git status
Miskin:   runs git status → captures output → applies filter → returns compact result
AI tool:  receives 200 tokens instead of 2000
```

### Filter strategies per command type:

| Strategy | Example | Applied to |
|----------|---------|------------|
| **OK-mode** | `git push` → `ok main` | git add/commit/push/pull/checkout/... |
| **Status grouping** | Staged/unstaged/untracked sections | git status |
| **Diff compaction** | Per-file +/-, 15-file limit | git diff |
| **Failure extraction** | Show only FAILED tests | cargo test, pytest, jest |
| **Error grouping** | Group by file | eslint, ruff, tsc |
| **Table compaction** | Show count + first N rows | docker ps, gh pr list |
| **Log dedup** | Collapse repeated lines | docker logs, kubectl logs |
| **Truncation** | Head/tail with skip count | cat, find, curl |

## 2. Caveman Prompt Injection

When enabled, Miskin injects "be concise" rules into the AI tool's system prompt. The model itself produces fewer output tokens:

```
Normal agent:  "Sure! I'd be happy to help. The issue is on line 42..."
Caveman agent:  "Bug L42. Fix: add null check."
Same fix. 65% fewer tokens.
```

Prompt is injected once per session via markdown files (CLAUDE.md, AGENTS.md, etc.) or plugin hooks depending on the AI tool.

## Hook Architecture

Miskin installs hooks that rewrite commands before execution:

| Agent | Hook type | File |
|-------|-----------|------|
| Claude Code | Bash hook script | `.claude/hooks/miskin-rewrite.sh` |
| Copilot | JS preToolUse hook | `.github/copilot-hooks/miskin-hook.js` |
| Cursor | hooks.json | `.cursor/hooks.json` |
| Gemini | settings hook | `.gemini/miskin-hook.json` |
| Codex | Instructions | `.codex/miskin.md` |
| OpenCode | TypeScript plugin | `.opencode/plugins/miskin.ts` |
| Windsurf | Rules file | `.windsurfrules` |

## Token Counting

Miskin uses `tiktoken-rs` (cl100k_base, same tokenizer as GPT-4/Claude) to count tokens accurately. Savings are recorded locally in `~/.local/share/miskin/analytics.json`.

## Tee Mode

When a command fails, Miskin saves the raw unfiltered output to `~/.local/share/miskin/tee/<timestamp>_<command>.log`. The AI tool can read this file to see full error output without re-executing the command.
