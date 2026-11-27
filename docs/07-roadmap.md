# 07 — Roadmap y guía de trabajo

> **Modo:** Daniel escribe el código. Yo reviso, cuestiono y desbloqueo.

## Principios de ejecución

- Un milestone = algo jugable, no una capa abstracta.
- Cada milestone termina con pruebas y un demo reproducible.
- No se avanza si el anterior no está verde.
- Los datos se pueden fakear al inicio, pero la **procedencia** se respeta desde
  el día 1.

## M0 — Repo y esqueleto (1–2 días)

**Entregable**
- Repo `opengaffer` con workspace Rust y `clients/tui`.
- CI mínimo: `cargo fmt`, `cargo clippy`, `cargo test`.
- `README` con cómo correr.

**DoD**
- `cargo build` y `cargo test` en verde.
- Un binario `gaffer-sim` que responde `ping` por JSONL.

## M1 — Catálogo mínimo (2–3 semanas)

**Entregable**
- Migraciones del catálogo con `clubs`, `teams`, `competitions`.
- Data pack de **México**: Liga MX (18), Sub-21 y Sub-19 espejo (18+18),
  Expansión (15/16).
- Importador desde una fuente abierta (Wikidata + API/fixtures).
- `gaffer-catalog build` reproducible y con checksum.

**DoD**
- Clubes, equipos, personas y competiciones cargados.
- Cada equipo tiene `club_id`, `gender`, `age_group` y `competition_id`.
- Cada dato tiene `source` y `confidence`.
- Reporte de conteos y validación básica.
- Premier, TDP y Femenil quedan como data packs posteriores (ver
  `09-piramide-mexico.md`).

## M2 — Motor y partido (2 semanas)

**Entregable**
- `gaffer-sim` con partido minuto a minuto.
- Calibración contra un set de referencia.
- Determinismo por seed.

**DoD**
- 1,000 partidos: goles/tiros/lesiones/rojas dentro de rango.
- Misma seed → mismo `trace_hash`.
- < 3 ms/partido en release.

## M3 — Estado y carrera (2–3 semanas)

**Entregable**
- `gaffer-state`: crear mundo, calendario, tabla, avanzar día.
- Lesiones, tarjetas, sanciones, ascenso/descenso.
- Journal + snapshots + replay.

**DoD**
- Temporada completa simulada.
- Crash y reopen con mismo `state_digest`.
- 50 reenvíos = 1 efecto.

## M4 — Servidor y multiplayer (2–3 semanas)

**Entregable**
- `gaffer-server` WebSocket con auth, lobby, seats, invites, sync.
- Host lease y takeover.
- Audit y logs.

**DoD**
- 2 clientes juegan; uno avanza y el otro recibe.
- Reclamo simultáneo → `conflict`.
- Late join con briefing.
- 100 reconexiones sin huecos.

## M5 — Cliente (2–3 semanas)

**Entregable**
- TUI: login, mundos, clubes libres, plantilla, táctica, partido, tabla.
- Reconexión visual y estados de error.

**DoD**
- Una temporada jugable de punta a punta con 2 personas.
- Un manager nuevo entiende su club en < 2 min.

## M6 — Datos reales y balance (continuo)

**Entregable**
- Ratings globales por capas.
- Cobertura tier A/B/C.
- Validación y ajuste.

**DoD**
- Distribuciones sanas por liga/posición.
- Transferencias entre ligas predicen rendimiento.
- Simulación continental coherente.

## Primeros 10 pasos concretos

1. Crea el repo y el workspace.
2. Define la taxonomía de atributos.
3. Escribe las migraciones del catálogo.
4. Lista la pirámide mexicana y separa `club` de `team`.
5. Conecta una fuente abierta y guarda el crudo.
6. Resuelve IDs (Wikidata) y arma el data pack de Liga MX + Sub-21/Sub-19.
7. Implementa el partido mínimo (sin tácticas).
8. Calibra goles/tiros/lesiones.
9. Crea el mundo y avanza una jornada.
10. Levanta el WebSocket y conecta dos clientes.

## Qué reutilizar del repo actual

| pieza | ruta | cómo usarla |
|---|---|---|
| motor | `engine-rs/crates/fm-sim` | referencia de comportamiento y calibración |
| estado | `engine-rs/crates/fm-state` | referencia de journal/snapshots/audit |
| servidor | `engine-rs/crates/fm-server` | referencia de WS/sync/auth |
| protocolo | `docs/PROTOCOLO_MOTOR.md` + `protocol/` | contrato base |
| TUI | `fm_tui/` | referencia de UX |
| extracción | `extract/` | adaptador opcional de saves |

## Cómo te guío

- Reviso tus PRs y diseños.
- Te hago preguntas de borde antes de que codees.
- Te doy criterios de aceptación y casos de prueba.
- No escribo código de producción; sí ejemplos mínimos si te atoras.

## Ejercicios para no oxidarte

- Implementa el emparejamiento round-robin y pruébalo con 18 equipos.
- Escribe el cálculo de tabla con desempates.
- Implementa un journal con hash chain y un test de corrupción.
- Diseña el comando `clubs.claim` con concurrencia optimista.
- Calcula el rating de 10 jugadores con datos reales y defiende el resultado.
