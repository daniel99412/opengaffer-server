# OpenGaffer — Server

Authoritative server for **OpenGaffer**, an open-source, multiplayer football
management simulator built on its own data. The long-term goal is a game that
can compete with Football Manager without depending on Sports Interactive's
database or assets.

This repository contains **the server only**: match engine, world state,
catalog and the network layer. The client lives in its own repository
(`opengaffer-client`).

## Status

**Initial skeleton (M0).** There is no game logic yet. The only binary exists
to validate that the workspace builds and runs:

```bash
cargo run -p gaffer-server
# Hello, OpenGaffer!
```

## Architecture

```
┌─────────────┐        WSS/JSON        ┌──────────────────────┐
│   Client    │ ─────────────────────► │  gaffer-server       │
│  (TUI/Web)  │ ◄───────────────────── │  auth · lobby · sync │
└─────────────┘   snapshots/events     └──────────┬───────────┘
                                                   │ commands
                                        ┌──────────▼───────────┐
                                        │  gaffer-state        │
                                        │  world · journal     │
                                        │  snapshots · audit   │
                                        └──────────┬───────────┘
                                                   │ simulates
                                        ┌──────────▼───────────┐
                                        │  gaffer-sim          │
                                        │  match engine        │
                                        └──────────────────────┘
```

| crate | responsibility | does not |
|---|---|---|
| `gaffer-core` | shared types, IDs, protocol | IO, network, DB |
| `gaffer-sim` | deterministic match engine | DB, network, clock |
| `gaffer-state` | world, commands, events, journal, snapshots | network |
| `gaffer-catalog` | ingestion, normalization, ratings, data packs | network at runtime |
| `gaffer-server` | transport, sessions, actors, auth | simulation formulas |

## Requirements

- Stable Rust with **edition 2024** support (1.85 or newer).
- No database is needed for the current skeleton.

## Running

```bash
# Build the whole workspace
cargo build --workspace

# Start the server
cargo run -p gaffer-server

# Tests
cargo test --workspace

# Formatting and lints
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Repository layout

```
opengaffer-server/
├── Cargo.toml                  # workspace
├── Cargo.lock
├── crates/
│   ├── gaffer-core/            # types and protocol
│   ├── gaffer-sim/             # match engine
│   ├── gaffer-state/           # world + persistence
│   ├── gaffer-catalog/         # ingestion + ratings + data packs
│   └── gaffer-server/          # server binary
├── catalog/                    # data packs and manifests
└── docs/                       # design documentation
```

## Documentation

Design documentation lives in [`docs/`](docs/README.md). Suggested reading
order (currently in Spanish):

1. [`00-vision.md`](docs/00-vision.md) — what OpenGaffer is and is not.
2. [`01-arquitectura.md`](docs/01-arquitectura.md) — components and stack.
3. [`02-modelo-datos.md`](docs/02-modelo-datos.md) — catalog vs world.
4. [`03-rating-global.md`](docs/03-rating-global.md) — stat calculation.
5. [`04-fuentes-datos.md`](docs/04-fuentes-datos.md) — data sources and licenses.
6. [`05-multijugador.md`](docs/05-multijugador.md) — worlds, seats, sync.
7. [`06-aprendizajes-fm.md`](docs/06-aprendizajes-fm.md) — FM24 technical notes.
8. [`07-roadmap.md`](docs/07-roadmap.md) — milestones and Definition of Done.
9. [`08-legal.md`](docs/08-legal.md) — trademarks, copyright and data.
10. [`09-piramide-mexico.md`](docs/09-piramide-mexico.md) — Mexican football pyramid.

## Principles

- **The server is authoritative.** Clients send intents; they never simulate state.
- **Determinism.** Same seed and same inputs produce the same result.
- **Every change is auditable.** `command_id`, `seq`, journal and snapshots.
- **Data provenance.** Every value carries source, license, method and confidence.
- **No Sports Interactive data or assets.** A user's FM save is imported locally
  and never redistributed.

## Roadmap

The milestone plan lives in [`docs/07-roadmap.md`](docs/07-roadmap.md).
Currently at **M0: repository and skeleton**.

## License

To be defined before publishing.
