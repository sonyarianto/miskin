# Linting Commands

## eslint / biome

```bash
$ miskin eslint .

src/main.ts
  1:5  error  'x' is never used  no-unused-vars
  3:2  warning  Missing semicolon  semi
src/lib.ts
  5:1  error  Unexpected console  no-console
```

## ruff

```bash
$ miskin ruff check .

src/main.py: F401, E501
src/lib.py: F841
```

Clean:

```bash
ruff ok
```

## tsc / mypy (type checkers)

```bash
$ miskin tsc

  src/main.ts (3 errors)
  src/lib.ts (2 errors)
```

Clean:

```bash
types ok
```

## prettier

```bash
$ miskin prettier --check .

3 files need formatting:
  src/main.ts
  src/lib.ts
```

Clean:

```bash
format ok
```

## Supported Linters

| Linter | Aliases |
|--------|---------|
| eslint | biome, golangci-lint, rubocop |
| ruff | — |
| tsc | mypy |
| prettier | — |
| clippy | (via `cargo clippy`, not `lint`) |
