# Commands

Miskin filters 40+ commands. Run any supported command through `miskin` to get compressed output.

```bash
miskin <command> [args...]
```

## Supported Commands

| Category | Commands |
|----------|----------|
| **Git** | status, diff, log, add, commit, push, pull, branch, checkout, merge, rebase, stash, tag, fetch, clone, init, remote, reset, restore, rm, mv, clean |
| **Rust** | cargo build, test, clippy, fmt, check, run |
| **Node.js** | npm, pnpm, yarn, npx, bun, pip, uv, bundle, gem |
| **Testing** | pytest, jest, vitest, go test, rspec, playwright |
| **Linting** | eslint, ruff, biome, tsc, mypy, prettier, golangci-lint, rubocop, clippy |
| **Docker/K8s** | docker ps, images, logs, compose; kubectl, oc |
| **Files** | ls, cat, find, tree, head, tail, read |
| **GitHub** | gh pr list/view/status, issue list, run list, repo view, auth |
| **Network** | curl, wget |
| **System** | df, du, ps, top, wc, env, which, uname, free |

## CLI Subcommands

```bash
miskin init               # Install hooks
miskin stats              # Token savings
miskin gain               # Alias for stats
miskin config             # Manage configuration
miskin compress           # Print caveman system prompt
miskin discover           # Find unfiltered commands
miskin session            # Adoption by session
miskin proxy <cmd>        # Raw passthrough + tracking
miskin err <cmd>          # Show only stderr
miskin completions bash   # Shell completions
miskin --help             # Full help
```

## See Also

- [Git commands](/commands/git)
- [Rust/Cargo commands](/commands/cargo)
- [Node.js/npm commands](/commands/npm)
- [Testing commands](/commands/testing)
- [Linting commands](/commands/linting)
- [Docker/K8s commands](/commands/docker)
- [Files commands](/commands/files)
- [GitHub CLI commands](/commands/gh)
- [System & Network commands](/commands/system)
