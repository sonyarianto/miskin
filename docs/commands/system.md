# System & Network Commands

## curl / wget

Large responses are truncated:

```bash
$ miskin curl https://api.example.com/large-response

OK (12456 bytes). First 5000 chars:
<first 30 lines>
```

Small responses pass through:

```bash
$ miskin curl https://api.example.com/health

{"status": "ok"}
```

## System Commands

```bash
$ miskin df -h
$ miskin du -sh src/
$ miskin ps aux | head
$ miskin wc -l src/**/*.rs
$ miskin env | grep PATH
$ miskin which rustc
$ miskin uname -a
$ miskin free -h
```

Short output (≤20 lines): passes through. Longer output: truncated to 30 lines.

## err — Show Only Errors

```bash
$ miskin err cargo build

error[E0308]: mismatched types
  --> src/main.rs:42:9
```

No errors:

```bash
$ miskin err cargo build

no errors
```

## proxy — Raw Passthrough + Tracking

```bash
$ miskin proxy any-command --with --any flags
```

Runs the command without filtering but still tracks token counts in analytics.
