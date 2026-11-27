# OpenGaffer — Server

Servidor autoritativo de **OpenGaffer**, un simulador de gestión de fútbol
_open source_, multijugador y con datos propios. El objetivo a largo plazo es
construir un juego que pueda competir con Football Manager sin depender de la
base de datos ni de los assets de Sports Interactive.

Este repositorio contiene **solo el servidor**: motor de partido, estado del
mundo, catálogo de datos y la capa de red. El cliente vive en su propio
repositorio (`opengaffer-client`).

## Estado

**Esqueleto inicial (M0).** Todavía no hay lógica de juego. El único binario
existe para validar que el workspace compila y corre:

```bash
cargo run -p gaffer-server
# Hello, OpenGaffer!
```

## Arquitectura

```
┌─────────────┐        WSS/JSON        ┌──────────────────────┐
│  Cliente    │ ─────────────────────► │  gaffer-server       │
│  (TUI/Web)  │ ◄───────────────────── │  auth · lobby · sync │
└─────────────┘   snapshots/eventos    └──────────┬───────────┘
                                                   │ comandos
                                        ┌──────────▼───────────┐
                                        │  gaffer-state        │
                                        │  mundo · journal     │
                                        │  snapshots · audit   │
                                        └──────────┬───────────┘
                                                   │ simula
                                        ┌──────────▼───────────┐
                                        │  gaffer-sim          │
                                        │  motor de partido    │
                                        └──────────────────────┘
```

| crate | responsabilidad | no hace |
|---|---|---|
| `gaffer-core` | tipos compartidos, IDs, protocolo | IO, red, DB |
| `gaffer-sim` | motor de partido determinista | DB, red, reloj |
| `gaffer-state` | mundo, comandos, eventos, journal, snapshots | red |
| `gaffer-catalog` | ingesta, normalización, ratings, data packs | red en runtime |
| `gaffer-server` | transporte, sesiones, actores, auth | fórmulas de simulación |

## Requisitos

- Rust estable con soporte para **edition 2024** (1.85 o superior).
- No se necesita base de datos para el esqueleto actual.

## Cómo correr

```bash
# Compilar todo el workspace
cargo build --workspace

# Levantar el servidor
cargo run -p gaffer-server

# Pruebas
cargo test --workspace

# Formato y lints
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Estructura del repositorio

```
opengaffer-server/
├── Cargo.toml                  # workspace
├── Cargo.lock
├── crates/
│   ├── gaffer-core/            # tipos y protocolo
│   ├── gaffer-sim/             # motor de partido
│   ├── gaffer-state/           # mundo + persistencia
│   ├── gaffer-catalog/         # ingesta + ratings + data packs
│   └── gaffer-server/          # binario del servidor
├── catalog/                    # data packs y manifests
└── docs/                       # documentación de diseño
```

## Documentación

La documentación de diseño vive en [`docs/`](docs/README.md). Orden sugerido:

1. [`00-vision.md`](docs/00-vision.md) — qué es OpenGaffer y qué no.
2. [`01-arquitectura.md`](docs/01-arquitectura.md) — componentes y stack.
3. [`02-modelo-datos.md`](docs/02-modelo-datos.md) — catálogo vs mundo.
4. [`03-rating-global.md`](docs/03-rating-global.md) — cálculo de stats.
5. [`04-fuentes-datos.md`](docs/04-fuentes-datos.md) — fuentes y licencias.
6. [`05-multijugador.md`](docs/05-multijugador.md) — mundos, asientos, sync.
7. [`06-aprendizajes-fm.md`](docs/06-aprendizajes-fm.md) — notas técnicas de FM24.
8. [`07-roadmap.md`](docs/07-roadmap.md) — milestones y DoD.
9. [`08-legal.md`](docs/08-legal.md) — marcas, copyright y datos.
10. [`09-piramide-mexico.md`](docs/09-piramide-mexico.md) — pirámide mexicana.

## Principios

- **El servidor es la autoridad.** El cliente manda intents; no simula estado.
- **Determinismo.** Misma semilla y mismas entradas producen el mismo resultado.
- **Todo cambio es auditable.** `command_id`, `seq`, journal y snapshots.
- **Procedencia de datos.** Cada dato tiene fuente, licencia, método y confianza.
- **Sin datos ni assets de Sports Interactive.** El save de FM del usuario se
  importa localmente y nunca se redistribuye.

## Roadmap

El plan por milestones está en [`docs/07-roadmap.md`](docs/07-roadmap.md).
Actualmente en **M0: repo y esqueleto**.

## Licencia

Por definir antes de publicar.
