---
layout: home

hero:
  name: "Miskin"
  text: "Save AI tokens."
  tagline: Local. No server. Compresses command output + injects caveman-mode prompts.
  actions:
    - theme: brand
      text: Get Started
      link: /guide/
    - theme: alt
      text: View on GitHub
      link: https://github.com/sonyarianto/miskin

features:
  - icon: ⚡
    title: Command Filter
    details: Run any command through format-specific compressors. git, cargo, npm, docker — 40+ commands supported. 80%+ token reduction.
  - icon: 🗿
    title: Caveman Mode
    details: Inject "be concise" prompts into your AI tool. Same answers, 65% fewer output tokens. Four compression levels.
  - icon: 🔌
    title: Multi-Agent
    details: Hooks for Claude Code, Copilot, Cursor, Gemini, Codex, OpenCode, Windsurf. One install command.
  - icon: 📊
    title: Analytics
    details: Track token savings per command, per session, per day. ASCII graphs, JSON export. No telemetry, all local.
  - icon: 🦀
    title: Rust
    details: Single ~7MB binary. Zero runtime dependencies. <10ms overhead. Cross-platform Linux/macOS/Windows.
  - icon: 🔒
    title: Privacy First
    details: No telemetry. No accounts. No network calls. Everything stays on your machine.
---

## Quick Start

```bash
# Install
npm install -g miskin      # recommended
# or: cargo install miskin

# Set up hooks for your AI tool
miskin init               # Claude Code
miskin init --agent cursor
miskin init --agent copilot

# Restart your AI tool — commands are now auto-filtered
miskin git status          # 90% fewer tokens
miskin cargo test          # failures only
miskin docker ps           # compact table
```

## Approximate Savings

> **Note**: Rough projections, not benchmarks. Your numbers will vary. Run `miskin stats` to see real savings.

| Command | Typical raw | With Miskin | Approx. |
|---------|-------------|-------------|---------|
| `git status` | up to ~2,000 | ~200 | ~90% |
| `cargo test` | up to ~5,000 | ~500 | ~90% |
| `git diff` | up to ~1,000 | ~300 | ~70% |
| `eslint` | up to ~2,000 | ~400 | ~80% |
| `docker ps` | ~500 | ~100 | ~80% |
| `npm install` | ~1,000 | ~50 | ~95% |

## How It Works

```
Without Miskin:                     With Miskin:

git status ──▶ shell ──▶ LLM        git status ──▶ miskin ──▶ git ──▶ filter ──▶ LLM
  (~2000 tokens)                      (~200 tokens, 90% saved)
```

The hook rewrites `git status` to `miskin git status`. Miskin runs the real command, compresses output through format-aware filters, and returns compact results to the LLM.
