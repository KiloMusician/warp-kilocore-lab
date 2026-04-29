# KiloCore Warp Integration Lab

**Branch:** `kilocore/integration-lab`  
**Doctrine:** Config-first. No source divergence until two config-only cycles complete.  
**Upstream:** `warpdotdev/warp` (tracked on `upstream-main`)

## Status

| Layer | Status |
|-------|--------|
| Profile pack (themes, workflows, keybindings, rules) | Done — `Kilo_Core/docs/kilocore-profile/` |
| Warp fork + branch strategy | Done — this repo |
| Upstream-main sync | Manual (`git fetch upstream && git reset --hard upstream/master` on `upstream-main`) |
| Source-code features | Pending Rust build environment validation |

## Planned prototype features

1. **Status Bar Indicator** — shows KiloCore Gateway health + tool count in the Warp status bar
2. **MCP Tool Launcher Panel** — Warp Drive panel listing all 41 gateway tools with one-click invocation
3. **Agent Context Injector** — injects colony context (current task, active agent, recent audit entries) into new sessions

## Profile pack location

`C:/dev/active/Kilo_Core/docs/kilocore-profile/` — import into Warp via Settings.

## Branch rules

- `upstream-main` — pull-only mirror of `warpdotdev/warp`. Never push KiloCore commits here.
- `kilocore/integration-lab` — all KiloCore config/source changes. PRs merge here.
- `kilocore/upstream-prs` — staging for upstream contributions to evaluate or send back.

## Sync procedure

```bash
git checkout upstream-main
git fetch upstream
git reset --hard upstream/master
git push origin upstream-main

git checkout kilocore/integration-lab
git rebase upstream-main
```
