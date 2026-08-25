# warp-kilocore-lab - Claude Guidance

## Repo Role

This is an upstream Warp codebase used as a **local KiloCore integration lab**.

## Working Rules

- Keep local changes narrowly scoped and easy to reason about.
- Prefer small overlay patches over broad rewrites of upstream Warp behavior.
- Document colony-specific behavior at the repo root or in clearly local files.
- When a change is purely upstream Warp logic, avoid inventing extra colony
  doctrine around it.

## Good Fits

- local launch fixes
- channel/config fallback behavior
- integration notes for KiloCore surfaces
- small Rust or app-layer patches with clear local justification
