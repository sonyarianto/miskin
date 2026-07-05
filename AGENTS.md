# Agent Instructions

## Project

Miskin is a CLI proxy that saves AI tokens by compressing command output and injecting caveman-mode prompts. Single Rust binary. Zero runtime dependencies.

## Build & Test

```bash
cargo build              # Debug build
cargo build --release    # Release build (7.5MB)
cargo test               # Run all tests (47+)
cargo fmt --all --check  # Check formatting
cargo clippy -- -D warnings  # Lint
```

## Project Structure

```
src/
├── main.rs              # CLI entry, passthrough runner, streaming output
├── config.rs            # TOML config (general, filters, caveman, analytics)
├── tee.rs               # Raw output backup on failure
├── cli/                 # CLI subcommands
│   ├── mod.rs           # Clap definitions + all args
│   ├── init.rs          # Hook installation for 7 AI tools
│   ├── stats.rs         # Token savings reports (text, JSON, daily, graph)
│   ├── config_cmd.rs    # Show/set/reset config
│   ├── discover.rs      # Find unfiltered commands
│   └── session.rs       # Adoption by session
├── filters/             # Output compressors
│   ├── mod.rs           # CommandFilter trait + FilterRegistry
│   ├── generic.rs       # dedup, truncate, strip_ansi, group_by_ext/prefix
│   ├── git.rs           # status/diff/log/branch/ok commands
│   ├── cargo.rs         # build/test/clippy/fmt
│   ├── tests.rs         # pytest, jest, vitest, go test, rspec, playwright
│   ├── docker.rs        # ps, images, logs, compose; kubectl, oc
│   ├── files.rs         # ls, cat, find, tree
│   ├── lint.rs          # eslint, ruff, biome, tsc, mypy, prettier
│   ├── npm.rs           # npm, pnpm, yarn, npx, bun, pip, uv, bundle
│   ├── gh.rs            # GitHub CLI: pr, issue, run, repo, auth
│   ├── curl.rs          # curl, wget
│   └── system.rs        # df, du, ps, top, wc, env, which, uname, free
├── hooks/               # Per-tool hook installers
│   ├── mod.rs           # Path helpers
│   ├── claude.rs, copilot.rs, cursor.rs, gemini.rs, codex.rs, opencode.rs, windsurf.rs
├── prompt/mod.rs        # 4 caveman levels (lite/full/ultra/aggressive)
└── analytics/           # Token counting + local JSON storage
    ├── mod.rs           # AnalyticsStore
    ├── counter.rs       # tiktoken-rs token counting
    └── report.rs        # Summary + graph formatting
```

## Docs

```bash
cd docs && npm run dev     # Start VitePress dev server
cd docs && npm run build   # Build static site
```

Docs deploy to Vercel via `vercel.json`. Build dir: `docs/.vitepress/dist`.

## npm Package

```
packages/npm/
├── bin/miskin       # Shell script wrapper
├── install.js       # Downloads binary from GitHub Releases on postinstall
└── package.json     # bin → bin/miskin
```

To publish: tag triggers `.github/workflows/publish.yml` which builds binaries, creates GitHub Release, and runs `npm publish`.

## Releasing

1. Bump version in `Cargo.toml`
2. Bump version in `packages/npm/package.json` and `packages/npm/install.js`
3. Run `cargo check` to regenerate `Cargo.lock`
4. Run `cargo fmt --all --check`, `cargo clippy -- -D warnings`, `cargo test`
5. Commit all changes
6. Tag: `git tag v0.1.0` and `git push --tags`

CI will publish to:
- **crates.io** via `CARGO_REGISTRY_TOKEN`
- **npm** via `NPM_TOKEN`
- **GitHub Releases** automatically

## Adding a new filter

1. Create `src/filters/<name>.rs` implementing `CommandFilter` trait
2. Register in `FilterRegistry::default()` in `src/filters/mod.rs`
3. Add tests as `#[cfg(test)] mod unit_tests { ... }` at bottom of file
4. Add docs page under `docs/commands/`
5. Add to sidebar in `docs/.vitepress/config.mts`
6. Run `cargo fmt --all` and `cargo test`
