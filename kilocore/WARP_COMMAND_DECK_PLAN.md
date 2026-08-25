# WARP Command Deck Plan

Warp should act as a practical command deck for the colony, not a vague terminal fork.

## Pane Layout

| Pane | Purpose |
|---|---|
| 1 | `quick_colony_check.ps1` |
| 2 | Dev-Mentor cockpit logs |
| 3 | NuSyQ-Hub gateway / mission-board surface |
| 4 | Ollama + LiteLLM model routing checks |
| 5 | Goose attended lane |
| 6 | Pi bounded prompt lane |
| 7 | OpenClaw transcript lane |
| 8 | Serena dashboard / search lane |
| 9 | delegation planner |
| 10 | git status + receipts |

## Launch Proof Commands

```powershell
cd C:\dev\active\warp-kilocore-lab
$env:KILOCORE_WARP_DEV = "1"
cargo check

$env:RUST_BACKTRACE = "1"
$env:KILOCORE_WARP_DEV = "1"
.\target\debug\dev.exe
```

## Receipt Target

- `C:\dev\active\_agent_reports\warp\WARP_COMMAND_DECK_PROOF.md`

## Role

- foreground local cockpit
- pane-based operator awareness
- attended multi-lane execution surface
- not a hidden autonomy layer
