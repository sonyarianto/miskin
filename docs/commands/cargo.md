# Rust / Cargo Commands

## cargo test

Passing tests:

```bash
$ miskin cargo test

PASSED: 15/15 tests
```

Failing tests — shows only failures:

```bash
$ miskin cargo test

FAILED: 2/15

test_edge_case: assertion failed
test_overflow: panicked at 'assertion failed'
```

## cargo build

Success:

```bash
$ miskin cargo build

Compiling miskin v0.1.0
Finished dev [unoptimized] target(s) in 2.34s
```

Failure — shows error lines only:

```bash
error[E0308]: mismatched types
  --> src/main.rs:42:9
```

## cargo clippy

```bash
$ miskin cargo clippy

warning: unused variable `x` --> src/lib.rs:10
warning: redundant clone --> src/main.rs:5
```

Clean output:

```bash
clippy ok
```

## cargo fmt

```bash
$ miskin cargo fmt

2 files need formatting:
  src/main.rs
  src/lib.rs
```

Clean:

```bash
fmt ok
```
