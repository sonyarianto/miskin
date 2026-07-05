# Getting Started

Miskin is a CLI proxy that sits between your AI coding tool and your shell, compressing command output before it reaches the LLM's context window.

## 1. Install

### Cargo (recommended)

```bash
cargo install miskin
```

### From source

```bash
git clone https://github.com/sonyarianto/miskin
cd miskin
cargo install --path .
```

### Verify

```bash
miskin --version
```

## 2. Set Up Hooks

Hooks automatically rewrite shell commands before execution:

```bash
# Claude Code (default)
miskin init

# Other tools
miskin init --agent cursor
miskin init --agent copilot
miskin init --agent gemini
miskin init --agent codex
miskin init --agent opencode
```

Restart your AI tool after installing hooks.

## 3. Verify

Run any supported command in your AI tool:

```bash
git status
```

The hook rewrites it to `miskin git status`. You should see compact output.

## 4. Check Savings

```bash
miskin stats
miskin gain              # alias
miskin stats --graph     # per-command bar chart
miskin stats --daily     # day-by-day
miskin stats --json      # machine-readable
```

## 5. Enable Caveman Mode

Reduce LLM output tokens by 65%:

```bash
miskin config set caveman.enabled true
miskin config set caveman.level aggressive
```

This injects "be concise" instructions into your AI tool's system prompt. See [Configuration](/guide/configuration) for all levels.

## Shell Completions

```bash
# Bash
echo 'eval "$(miskin completions bash)"' >> ~/.bashrc

# Zsh
echo 'eval "$(miskin completions zsh)"' >> ~/.zshrc

# Fish
miskin completions fish > ~/.config/fish/completions/miskin.fish
```

## Dry Run

Preview what would be installed without writing anything:

```bash
miskin init --dry-run
miskin init --dry-run --agent cursor
miskin init --dry-run --hook-only
```

## Uninstall

```bash
miskin init --uninstall
miskin init --agent cursor --uninstall
cargo uninstall miskin
npm uninstall -g miskin
```
