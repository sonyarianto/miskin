# Miskin

**Save AI tokens across providers.** Single Rust binary that compresses command output and injects caveman-mode prompts. Zero dependencies, <10ms overhead.

```
Without Miskin:                     With Miskin:

git status ──▶ shell ──▶ LLM        git status ──▶ miskin ──▶ git ──▶ filter ──▶ LLM
  (~2000 tokens)                      (~200 tokens, 90% saved)
```

Inspired by [RTK](https://github.com/rtk-ai/rtk) and [Caveman](https://github.com/JuliusBrussee/caveman).

## Quick Start

```bash
# Build from source
cargo install --path .

# Install hooks for your AI tool
miskin init                # Claude Code (default)
miskin init --agent cursor
miskin init --agent copilot
miskin init --agent gemini
miskin init --agent codex
miskin init --agent opencode

# Restart your AI tool, then test
miskin git status          # 90% fewer tokens than raw git status
```

## How It Works

Two strategies, one binary:

| Strategy | What | Token savings |
|----------|------|---------------|
| **Command filter** | Run command, compress output before LLM sees it | 60-90% on dev commands |
| **Caveman prompt** | Inject "be concise" into system prompt | 65% on LLM responses |

The hook rewrites bash commands transparently:

```
git status    →  miskin git status
cargo test    →  miskin cargo test
ls -la        →  miskin ls -la
docker ps     →  miskin docker ps
```

Miskin runs the real command, filters the output through format-specific compressors,
and returns compact results. The LLM gets the same information in fewer tokens.

## Commands

```bash
# Passthrough mode — run any command through a filter
miskin git status
miskin git diff
miskin git log --oneline -10
miskin cargo test
miskin cargo build
miskin cargo clippy
miskin npm install
miskin docker ps
miskin ls -la
miskin cat src/main.rs
miskin pytest
miskin jest
miskin go test
miskin eslint .
miskin ruff check .
miskin curl https://api.example.com

# Subcommands
miskin init              # Install hooks
miskin init --show       # Verify installation
miskin stats             # Token savings report
miskin gain              # Alias for stats
miskin stats --daily     # Day-by-day breakdown
miskin stats --graph     # ASCII bar chart
miskin stats --json      # JSON export
miskin discover          # Find unfiltered commands
miskin session           # Adoption across sessions
miskin compress          # Print caveman system prompt
miskin config show       # View configuration
miskin config set caveman.level aggressive
miskin proxy <cmd>       # Raw passthrough + tracking
miskin err <cmd>         # Show errors only
miskin completions bash  # Generate shell completions
```

## Installation

### npm (recommended)

```bash
npm install -g miskin
```

### Cargo

```bash
cargo install --git https://github.com/sonyarianto/miskin
```

### From source

```bash
git clone https://github.com/sonyarianto/miskin
cd miskin
cargo install --path .
```

### Shell completions

```bash
# Bash
echo 'eval "$(miskin completions bash)"' >> ~/.bashrc

# Zsh
echo 'eval "$(miskin completions zsh)"' >> ~/.zshrc

# Fish
miskin completions fish > ~/.config/fish/completions/miskin.fish
```

## Supported Commands (40+)

| Category | Commands |
|----------|----------|
| **Git** | status, diff, log, add, commit, push, pull, branch, checkout, merge, rebase, stash, tag, fetch, clone, init, remote, reset, restore, rm, mv, clean |
| **Rust** | cargo build, test, clippy, fmt, check, run |
| **Node.js** | npm, pnpm, yarn, npx, bun, pip, uv, bundle, gem |
| **Testing** | pytest, jest, vitest, go test, rspec, playwright |
| **Linting** | eslint, ruff, biome, tsc, mypy, prettier, golangci-lint, rubocop, clippy |
| **Docker/K8s** | docker ps, images, logs, compose; kubectl, oc |
| **Files** | ls, cat, find, tree, head, tail, read |
| **Network** | curl, wget |
| **System** | df, du, ps, top, wc, env, which, uname, free |

## Approximate Savings

> **Note**: These are rough projections based on typical output sizes, not measured benchmarks. Actual savings depend on project size, command flags, and output length. Run `miskin stats` to see your real numbers.

| Command | Typical raw | With Miskin | Approx. saved |
|---------|-------------|-------------|---------------|
| `git status` | up to ~2,000 | ~200 | ~90% |
| `git diff` | up to ~1,000 | ~300 | ~70% |
| `git push` | ~200 | ~20 | ~90% |
| `cargo test` | up to ~5,000 | ~500 | ~90% |
| `ls -la` | ~400 | ~100 | ~75% |
| `docker ps` | ~500 | ~100 | ~80% |
| `eslint` | up to ~2,000 | ~400 | ~80% |
| `npm install` | ~1,000 | ~50 | ~95% |

Run `miskin stats` to see your actual numbers.

## Caveman Modes

Inject into system prompts to make the LLM produce fewer tokens:

| Level | Description |
|-------|-------------|
| `lite` | Concise. No pleasantries. |
| `full` | Caveman speak. Fragments. No meta. (default) |
| `ultra` | Absolute minimum. Drop articles. One-word when possible. |
| `aggressive` | Ultra + code-body stripping in explanations. |

```bash
miskin config set caveman.enabled true
miskin config set caveman.level ultra
miskin compress              # Preview the system prompt
```

## Configuration

`~/.config/miskin/config.toml`:

```toml
[general]
enabled = true
ultra_compact = false
exclude_commands = ["curl"]  # Skip filtering for these

[filters]
max_lines = 200
deduplicate = true

[caveman]
enabled = false
level = "full"

[analytics]
enabled = true
data_dir = "/home/user/.local/share/miskin"
```

## Analytics

Miskin tracks anonymized command token counts locally. No telemetry. No network calls.

```bash
miskin stats              # Summary
miskin stats --graph      # Per-command bar chart
miskin stats --daily      # Day-by-day
miskin stats --json       # JSON export
miskin discover           # Commands not yet filtered
miskin session            # Adoption by session
```

Data stored in `~/.local/share/miskin/analytics.json`.

## Supported AI Tools

| Tool | Command |
|------|---------|
| Claude Code | `miskin init` |
| GitHub Copilot | `miskin init --agent copilot` |
| Cursor | `miskin init --agent cursor` |
| Gemini CLI | `miskin init --agent gemini` |
| Codex | `miskin init --agent codex` |
| OpenCode | `miskin init --agent opencode` |
| Windsurf | `miskin init --agent windsurf` |

## Uninstall

```bash
miskin init --uninstall           # Remove hooks
miskin init --agent cursor --uninstall
cargo uninstall miskin
```

## License

MIT
