# File Commands

## ls

```bash
$ miskin ls

3 entries
main.rs
lib.rs
Cargo.toml
```

Empty directories:

```bash
empty
```

With directories:

```
dirs (2)
  src/
  tests/
files (8)
  *.rs (5 files)
  *.toml (1 file)
```

## cat / read

Files ≤50 lines: full content. Longer files truncate with head/tail:

```bash
$ miskin cat src/main.rs

<first 40 lines>
...
(60 lines omitted)
...
<last 40 lines>
```

## find

```bash
$ miskin find . -name '*.rs'

src/main.rs
src/lib.rs
src/filters/git.rs
src/filters/cargo.rs
...
```

30+ results show file count and common prefix grouping:

```bash
$ miskin find . -name '*.rs'

42 files
src/ (15 files)
src/filters/ (10 files)
tests/ (8 files)
...
```

Empty:

```bash
no matches
```

## tree

```bash
$ miskin tree

3 dirs, 8 files
├── src
│   ├── main.rs
│   └── lib.rs
├── tests
│   └── integration.rs
└── Cargo.toml
```
