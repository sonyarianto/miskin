# Git Commands

## status

```bash
$ miskin git status

On branch main
Changes (2):
  [unstaged] modified:   src/main.rs
  [unstaged] modified:   src/lib.rs
Untracked (1):
  src/new.rs
```

Clean repos show just `clean`.

## diff

```bash
$ miskin git diff

3 files (+12/-5)
  src/main.rs (+5/-2)
  src/lib.rs (+7/-3)
  Cargo.toml (+0/-0)
```

Diffs with more than 15 files truncate with a summary.

## log

```bash
$ miskin git log --oneline -10

abc1234 Fix: auth middleware
def5678 Add: user tests
ghi9012 Refactor: extract handler
```

More than 20 commits truncate with `... (N more commits)`.

## OK Commands

These commands show minimal output when successful:

| Command | Success Output |
|---------|---------------|
| `add` | `ok add` |
| `commit` | `[main abc1234]` |
| `push` | `ok push main` |
| `pull` | `ok pull 3 files +10 -2` |
| `checkout` | `ok checkout` |
| `merge`, `rebase`, `stash`, `tag`, `fetch` | `ok <command>` |
| `clone`, `init`, `remote`, `reset`, `restore`, `rm`, `mv`, `clean` | `ok <command>` |

## branch

```bash
$ miskin git branch

* main (+2 branches)
```
