# Node.js / npm Commands

## npm install / pnpm install / yarn add

```bash
$ miskin npm install

added 42 packages in 2s
```

Or: `install ok`

## npm run / yarn run

Extracts the last 3 lines of output for successful runs:

```bash
$ miskin npm run build

Build complete.
Ready for deployment.
```

## npm list / pnpm list

```bash
$ miskin npm list

3 packages
react@18.2.0
lodash@4.17.1
vue@3.4.0
```

## npm outdated / pnpm outdated

```bash
$ miskin npm outdated

all up to date
```

## Supported Package Managers

| Command | Aliases |
|---------|---------|
| npm | pnpm, yarn, npx, bun |
| pip | uv |
| bundle | gem |
