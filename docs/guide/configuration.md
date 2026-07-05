# Configuration

Miskin reads configuration from `~/.config/miskin/config.toml`. Run `miskin config show` to see current config, `miskin config reset` to restore defaults.

## Full Schema

```toml
[general]
enabled = true
ultra_compact = false
exclude_commands = ["curl", "wget"]

[filters]
max_lines = 500
strip_comments = false
deduplicate = true

[caveman]
enabled = false
level = "full"

[analytics]
enabled = true
data_dir = "/home/user/.local/share/miskin"
```

## General

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable/disable all filtering |
| `ultra_compact` | bool | `false` | ASCII icons, inline format |
| `exclude_commands` | string[] | `[]` | Commands to skip (passed through raw) |

## Filters

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `max_lines` | number | `500` | Max lines before truncation for unrecognized commands |
| `strip_comments` | bool | `false` | Strip code comments from output |
| `deduplicate` | bool | `true` | Remove duplicate output lines |

## Caveman Mode

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Enable caveman prompt injection |
| `level` | string | `"full"` | Compression level: `lite`, `full`, `ultra`, `aggressive` |

### Levels

| Level | Description |
|-------|-------------|
| `lite` | Concise. No pleasantries. |
| `full` | Caveman speak. Fragments. No meta. *(default)* |
| `ultra` | Absolute minimum. Drop articles. One-word answers. |
| `aggressive` | Ultra + code-body stripping. Diffs shown as `-old / +new`. |

View the prompt for any level with `miskin compress`.

## Analytics

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Record anonymous token counts |
| `data_dir` | string | `~/.local/share/miskin` | Where analytics JSON is stored |

All analytics are local-only. No telemetry. No network calls. Ever.

## CLI Commands

```bash
miskin config show                     # View current config
miskin config set general.ultra_compact true
miskin config set caveman.level ultra
miskin config set filters.max_lines 100
miskin config reset                    # Restore defaults
```
