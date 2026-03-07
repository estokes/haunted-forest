# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Haunted Forest is a sci-fi space game built with Bevy (0.18) and Rust (edition 2024). The player operates a starship through layered simulations connected by a reactive data fabric. Ship systems are physics simulations whose state is exposed through netidx (in-process pub/sub), with interfaces built in Graphix (a reactive dataflow language). There is no privileged game UI layer — everything connects through the same netidx hierarchy.

## Build & Run

```bash
cargo build          # build (uses opt-level="s" + thin LTO in dev)
cargo run            # run the game
cargo check          # type-check without building
cargo clippy         # lint
cargo test           # run tests
cargo test <name>    # run a single test by name
```

**Path dependencies**: The project depends on local checkouts of `netidx` (at `../netidx/`) and `graphix` (at `../graphix/`). These must be present for compilation. The path deps must match graphix's workspace to ensure type identity across crates.

## Architecture

### Four-Layer Design

1. **Ship Systems (Bevy ECS)** — Physics simulations running at `FixedUpdate`. Each system publishes state and accepts control through netidx. The reactor (`src/ship/reactor.rs`) is the first implemented system.

2. **netidx (Data Fabric)** — In-process pub/sub hierarchy at paths like `/ship/reactor/plasma/temperature`. Values can be read-only or writable (bidirectional control). The `NetBridge` (`src/net/bridge.rs`) connects Bevy's sync ECS to netidx's async runtime via crossbeam channels.

3. **Graphix (Reactive UI/Scripting)** — Reactive dataflow language for building interfaces and player scripts. Runs as a tokio task sharing the same runtime and netidx pub/sub as the NetBridge. Integration in `src/graphix/`.

4. **LLM (Character Interactions)** — Not yet implemented. Will use a local model via `llama-cpp-2` on a separate thread with crossbeam channel token streaming.

### Key Resource Flow

`NetPlugin` must be added before `GraphixPlugin` (Graphix needs the `NetInfra` resource). Plugin order in `main.rs` matters.

```
NetPlugin  →  inserts NetInfra + NetBridge resources
GraphixPlugin  →  reads NetInfra, inserts GraphixRuntime
PlayerPlugin  →  first-person camera + WASD movement
ShipPlugin  →  reactor simulation, HUD, keyboard controls
```

### Async Bridge Pattern

The project bridges Bevy (sync, single-threaded ECS) to tokio (async) using:
- A tokio `Runtime` owned by `NetInfra` (lives as a Bevy `Resource`)
- `crossbeam_channel` for sync↔async communication
- The bridge task polls commands from Bevy every 16ms and batches publisher updates
- Graphix uses a tokio mpsc → crossbeam bridge for events back to Bevy

### netidx Path Convention

Ship state is published under `/ship/<system>/<subsystem>/<value>`. Writable control paths go under `.../control/`. The hierarchy mirrors the ship's physical structure. See `book/architecture.md` for the full path tree.

## Source Layout

- `src/main.rs` — App setup, plugin registration, `GameState` enum
- `src/net/` — `NetInfra` (tokio runtime + netidx), `NetBridge` (pub/sub/write bridge)
- `src/graphix/` — Graphix runtime integration, stdlib package registration
- `src/ship/` — Ship systems: `reactor.rs` (physics + auto-control + publish), `bridge.rs` (3D bridge room), `hud.rs` (reactor HUD via netidx subscriptions)
- `src/player/` — First-person controller (camera, movement, cursor grab)
- `book/` — World bible and design docs (setting, architecture, ship systems, narrative)

## World & Design Docs

The `book/` directory contains the game's world bible and technical design docs. `book/architecture.md` is the authoritative reference for the game's technical design — read it before making architectural decisions. `book/ship-systems.md` covers the physics and engineering that the simulations model.
