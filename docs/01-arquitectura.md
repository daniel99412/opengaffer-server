# 01 — Arquitectura

## Vista general

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

## Componentes

| componente | responsabilidad | no hace |
|---|---|---|
| **catálogo** | datos de referencia versionados (clubes, personas, reglas) | no guarda estado de partida |
| **gaffer-state** | mundo, temporada, tabla, finanzas, journal, snapshots, audit | no hace red |
| **gaffer-sim** | simula un partido | no toca DB ni red |
| **gaffer-server** | auth, sesiones, lobby, sync, rate limit | no simula ni escribe DB directo |
| **cliente** | render, input, predicción visual | no modifica estado |

## Stack recomendado

- **Rust** para `gaffer-sim`, `gaffer-state`, `gaffer-server`.
  - `tokio` + `axum` (WebSocket), `serde`, `rusqlite` (o `sqlx` + Postgres),
    `tracing`, `sha2`, `zstd`.
- **Cliente inicial**: TUI en Python/Textual (rápido de iterar) o Rust + `ratatui`.
  - Regla: el cliente solo habla el protocolo; no abre la DB.
- **Control plane** (cuando haya varios mundos): Postgres para cuentas,
  catálogo de mundos y presencia. Un mundo = su propia DB y su actor.

### ¿Por qué no un solo binario?

- El motor se prueba sin red ni DB.
- El servidor no arrastra el motor a su espacio de fallas.
- Puedes correr `gaffer-sim` en lote para calibrar.
- El día que quieras web/móvil, solo hablan el protocolo.

## Repositorio propuesto

```
opengaffer/
├── Cargo.toml                # workspace
├── crates/
│   ├── gaffer-core/          # tipos compartidos (IDs, dominio)
│   ├── gaffer-sim/           # motor de partido
│   ├── gaffer-state/         # mundo + persistencia
│   ├── gaffer-server/        # red
│   └── gaffer-catalog/       # ingesta + ratings + data packs
├── clients/
│   └── tui/                  # cliente (Python o Rust)
├── catalog/
│   ├── migrations/
│   └── data/                 # data packs (no versionar binarios grandes)
├── docs/
└── tools/
```

## Flujo de un comando

1. Cliente envía `{id, cmd, command_id, expected_version, params}`.
2. Servidor valida auth, sesión, asiento y rate limit.
3. `gaffer-state` abre transacción `BEGIN IMMEDIATE`.
4. Dedupe por `command_id` (si ya existe, devuelve resultado).
5. Valida `expected_version` (si no, `conflict` con estado actual).
6. Aplica la mutación.
7. Si hay partidos, llama a `gaffer-sim` (batch).
8. Escribe `seq`, journal, audit y `command_log` en la **misma** transacción.
9. `COMMIT` (fsync) y **después** responde.
10. Broadcast de eventos a los clientes conectados.

## Flujo de reconexión

1. Cliente reconecta con `sync.resume {last_seq}`.
2. Si el journal cubre `last_seq`: manda delta de eventos.
3. Si no: manda `snapshot {seq, state, checksum}` + eventos posteriores.
4. Cliente descarta su estado y aplica; verifica checksum.

## Separación catálogo / mundo

- **Catálogo**: inmutable por versión. Se construye desde fuentes y se publica
  como data pack con checksum. Un mundo lo referencia (`catalog_version`).
- **Mundo**: mutable. Su DB contiene solo lo que cambia: temporada, partidos,
  tabla, finanzas, lesiones, journal.

Esto permite:
- Mundos con distintas reglas o países sin duplicar jugadores.
- Actualizar el catálogo sin romper mundos existentes.
- Backup de un mundo sin arrastrar 400 MB de referencia.

## Estrategia de pruebas

| nivel | qué prueba | herramienta |
|---|---|---|
| unitario | reglas, ratings, fechas, tabla | Rust tests |
| motor | calibración y determinismo | set dorado + distribuciones |
| estado | journal, snapshot, replay, idempotencia | tests de durabilidad |
| servidor | WS, auth, sync, 2+ clientes | integración tokio |
| caos | crash, duplicados, reconexión | inyección de fallas |
| end-to-end | temporada completa | script de simulación |

## Decisiones abiertas (para ti)

- ¿Cliente inicial en Python/Textual o Rust/ratatui?
- ¿SQLite por mundo o Postgres desde el día 1?
- ¿Un país en v1: México o Inglaterra?
- ¿Ritmo por defecto: asíncrono o programado?
