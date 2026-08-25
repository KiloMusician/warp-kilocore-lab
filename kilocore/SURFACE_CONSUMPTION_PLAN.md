# Warp Surface Consumption Plan

Warp should consume the generated KiloCore contract bundle first, then fall back to the index if needed.

## Preferred Input

- `C:/dev/active/Kilo_Core/config/coding_colony/surface_contract_registry.generated.json`

## Fallback Inputs

- `C:/dev/active/Kilo_Core/config/coding_colony/surface_contract_index.json`
- repo-local contract files listed there

## Why

This keeps Warp source work simpler:

- one bundle for cards/panels
- one source of truth for API vs MCP vs both
- cleaner future tool-launch and health-card UX

## First Warp Consumer Features

- dual-plane surface cards
- workflow launcher grouped by API/MCP/both
- auth-state warnings from the integration auth registry
- event-tail pane recommendations by surface
