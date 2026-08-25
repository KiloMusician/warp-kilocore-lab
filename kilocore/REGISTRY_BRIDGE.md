# Warp Registry Bridge

Warp should consume the colony registry artifacts instead of relying on bespoke workflow assumptions.

## Inputs

Primary registry inputs:

- `C:/dev/active/Kilo_Core/config/coding_colony/service_mcp_registry.json`
- `C:/dev/active/Kilo_Core/config/coding_colony/integration_auth_registry.json`
- `C:/dev/active/Kilo_Core/config/coding_colony/workflow_registry.json`

Secondary surface contracts:

- `C:/dev/active/NuSyQ-Hub/config/dual_plane_contract.json`
- `C:/dev/active/Dev-Mentor/config/dual_plane_contract.json`

## Warp Role

Warp should:

- read workflow metadata from `Kilo_Core`
- present operator workflows
- surface status/event tails
- keep AI-facing actions on MCP
- keep service state on API

## Recommended Next Source-Level Features

- status bar gateway health
- registry-driven MCP tool launcher
- dual-plane surface cards
- event tail pane templates
