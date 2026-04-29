# KiloCore Warp Integration Lab

**Branch:** `kilocore/integration-lab`  
**Doctrine:** Config-first. No source divergence until two config-only cycles complete.  
**Upstream:** `warpdotdev/warp` (tracked on `upstream-main`)

## Status

| Layer | Status |
|-------|--------|
| Profile pack (themes, workflows, keybindings, rules) | Done — `Kilo_Core/docs/kilocore-profile/` |
| `Kilo_Core/.warp/` — full colony Drive project | Done — 20 workflows, 2 rules, theme, 3 launch configs, mcp_servers.json |
| Fork `.warp/` — colony workflows + rules + theme + MCP config | Done — mirrored from colony Drive |
| Global keybindings | Done — 9 colony bindings in `AppData/Local/warp/Warp/config/keybindings.yaml` |
| Startup tab config | Done — colony + pi + openclaw + events tail panes |
| `crates/kilocore_gateway` Rust crate | Stub done — `GatewayClient`, `GatewayStatus`, `status_probe` example |
| Warp fork + branch strategy | Done — this repo |
| Upstream-main sync | Manual — see sync procedure below |

## Kilo_Core `.warp/` Workflow Inventory

| Workflow | Description |
|----------|-------------|
| `colony-up` | Bring up full Docker stack, wait for gateway |
| `colony-down` | Gracefully stop the stack |
| `colony-lattice-up` | Start optional lattice services (Gordon, Serena, etc.) |
| `colony-doctor` | Full health check: Docker, gateway, inference |
| `gateway-check` | Run `gateway_check.py` — services + tool count + self-test |
| `gateway-tools-count` | Live MCP tool count via JSON-RPC |
| `dev-mentor-status` | Dev-Mentor HTTP + MCP + recent game events |
| `nusyq-hub-status` | NuSyQ-Hub health + active missions |
| `agent-dispatch` | Route a task to a colony agent |
| `ollama-query` | Run a prompt through local Ollama |
| `ollama-list-models` | List installed Ollama models |
| `continuity-log` | Log an architectural decision |
| `continuity-search` | Search past colony decisions |
| `swarm-status` | Show active swarm tasks from Dev-Mentor |
| `audit-tail` | Tail the MCP gateway audit log |
| `colony-events-tail` | Follow the colony visual event bus |
| `colony-visual-ambience` | Start TTE banner + term-pet + nms tmux session |
| `emit-colony-event` | Push a structured event into the bus |
| `git-colony-status` | Git status across all active repos |
| `secret-hygiene` | Scan .env files for exposed key families |
| `warp-fork-build` | Build the Warp fork |
| `warp-fork-sync` | Sync upstream and rebase integration-lab |

## `crates/kilocore_gateway`

Lightweight stdlib-only gateway client crate. No async runtime required — designed for status bar polling.

```
src/
  lib.rs         — re-exports GatewayClient, GatewayStatus, ToolInfo
  client.rs      — GatewayClient: probe(), list_tools(), raw HTTP/1.1
  status.rs      — GatewayStatus, ServiceHealth, ToolInfo, label()
examples/
  status_probe.rs — CLI exerciser: prints status bar label + service table
```

**Run the probe:**
```bash
cd C:/dev/active/warp-kilocore-lab
cargo run --package kilocore_gateway --example status_probe
# With remote host:
KILOCORE_GATEWAY_HOST=192.168.1.5 cargo run --package kilocore_gateway --example status_probe
```

**Planned status bar integration:**
- `GatewayStatus::label()` → Warp status bar right segment
- Poll interval: 30s background thread
- Click to open colony-doctor workflow

## Planned prototype features (next config cycle)

1. **Status Bar Widget** — wire `GatewayStatus::label()` into Warp's right status bar segment; poll every 30s
2. **MCP Tool Launcher Panel** — Warp Drive panel listing all 41+ gateway tools with one-click invocation; powered by `list_tools()`
3. **Agent Context Injector** — inject colony context (current task, active agent, recent audit entries) into new agent sessions via rules + env vars
4. **Colony Event Pane** — persistent pane showing `tail_colony_events.py --follow` output, styled with KiloCore Dark theme

## MCP Servers (`.warp/mcp_servers.json`)

Three servers configured in both `Kilo_Core/.warp/` and the fork's `.warp/`:

| Server | Type | Endpoint |
|--------|------|----------|
| `kilocore-gateway` | HTTP | `http://localhost:8000/mcp` |
| `terminal-depths` | stdio | `python -u C:/dev/active/Dev-Mentor/mcp/server.py` |
| `continuity` | stdio | `python -u .../launch_continuity_mcp.py` |

## Profile pack location

`C:/dev/active/Kilo_Core/docs/kilocore-profile/` — import into Warp via Settings.

The live Drive config is `C:/dev/active/Kilo_Core/.warp/` — Warp picks this up automatically when the working directory is inside `Kilo_Core`.

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
