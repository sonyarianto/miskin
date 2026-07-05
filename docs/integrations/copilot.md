# GitHub Copilot

## Install

```bash
miskin init --agent copilot          # Local
miskin init -g --agent copilot       # Global
```

## What gets installed

| File | Purpose |
|------|---------|
| `.github/copilot-hooks/miskin-hook.js` | preToolUse hook |
| `.github/MISKIN.md` | Caveman prompt |

## How the hook works

```js
module.exports = {
  preToolUse: async (toolCall) => {
    if (toolCall.name === 'execute_command' || toolCall.name === 'bash') {
      const cmd = toolCall.arguments?.command || '';
      const base = cmd.split(' ')[0];
      if (['git', 'cargo', 'npm', ...].includes(base)) {
        toolCall.arguments.command = `miskin ${cmd}`;
      }
    }
    return toolCall;
  }
};
```

## Enable Caveman Mode

```bash
miskin config set caveman.enabled true
```

This writes caveman instructions to `.github/MISKIN.md`.

## Limitations

Copilot's VS Code extension has limited hook support compared to Claude Code. The CLI version (`gh copilot`) has better hook support.

## Uninstall

```bash
miskin init --agent copilot --uninstall
```
