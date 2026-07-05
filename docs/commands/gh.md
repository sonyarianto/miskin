# GitHub CLI Commands

## gh pr list

```bash
$ miskin gh pr list

3 PRs
  #123 Fix auth middleware bug
  #456 Add user profile tests
  #789 Refactor database layer
```

Empty:

```bash
no open PRs
```

## gh pr view

```bash
$ miskin gh pr view 123

title: Fix auth middleware bug
state: OPEN
author: sony
base: main
✓ CI / test
✓ CI / lint
```

## gh pr status

```bash
$ miskin gh pr status

My PRs (2):
  #123 Fix auth middleware bug
  #456 Add user tests
Needs review (1):
  #789 Update deps
```

## gh issue list

```bash
$ miskin gh issue list

2 issues
  #10 Database connection pool leak
  #11 Add rate limiting middleware
```

## gh run list

```bash
$ miskin gh run list

5 runs
  ✗ 1 failed:
    Deploy (abc1234)
  ○ 1 running:
    Release (def5678)
  ✓ 3 passed
```

## gh repo view

```bash
$ miskin gh repo view

name: sonyarianto/miskin
description: Save AI tokens across providers
visibility: public
default branch: main
```

## gh auth

```bash
$ miskin gh auth status

Logged in to github.com as sony
```
