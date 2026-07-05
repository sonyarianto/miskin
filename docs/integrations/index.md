# Integrations

Miskin supports 7 AI coding tools with automatic hook installation.

| Tool | Command | Hook method |
|------|---------|-------------|
| Claude Code | `miskin init` | PreToolUse bash hook |
| GitHub Copilot | `miskin init --agent copilot` | preToolUse JS hook |
| Cursor | `miskin init --agent cursor` | hooks.json preToolUse |
| Gemini CLI | `miskin init --agent gemini` | BeforeTool hook |
| Codex | `miskin init --agent codex` | AGENTS.md instructions |
| OpenCode | `miskin init --agent opencode` | TypeScript plugin |
| Windsurf | `miskin init --agent windsurf` | .windsurfrules file |

## Global vs Local

- **Local** (default): hooks install in the current project directory
- **Global** (`-g`): hooks install in `~/.config/<tool>/`

```bash
miskin init -g               # Global Claude Code hook
miskin init -g --agent cursor # Global Cursor hook
```

## Verify

```bash
miskin init --show            # Show installed hooks for Claude
miskin init --agent cursor --show
```

## Uninstall

```bash
miskin init --uninstall
miskin init --agent cursor --uninstall
```

## See Also

- [Claude Code](/integrations/claude)
- [GitHub Copilot](/integrations/copilot)
- [Cursor](/integrations/cursor)
- [Gemini CLI](/integrations/gemini)
- [Codex](/integrations/codex)
- [OpenCode](/integrations/opencode)
- [Windsurf](/integrations/windsurf)
