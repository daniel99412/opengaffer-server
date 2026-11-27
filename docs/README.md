# OpenGaffer — documentación de arranque

Estos documentos son la base para empezar **OpenGaffer desde cero**: un juego de
gestión de fútbol open source, multiplayer, con datos propios y sin depender de
la base de Sports Interactive.

> **Modo de trabajo:** Daniel escribe el código. Esta documentación es la guía:
> decisiones, modelos, fórmulas, checklists y criterios de aceptación. No hay
> código de producción aquí.

## Orden de lectura

1. [`00-vision.md`](00-vision.md) — qué es OpenGaffer, alcance y no-objetivos.
2. [`01-arquitectura.md`](01-arquitectura.md) — componentes, stack y flujo.
3. [`02-modelo-datos.md`](02-modelo-datos.md) — catálogo vs mundo, tablas, claves.
4. [`03-rating-global.md`](03-rating-global.md) — cómo calcular stats balanceadas.
5. [`04-fuentes-datos.md`](04-fuentes-datos.md) — de dónde salen los datos y licencias.
6. [`05-multijugador.md`](05-multijugador.md) — mundos, asientos, invitaciones, sync.
7. [`06-aprendizajes-fm.md`](06-aprendizajes-fm.md) — volcado técnico de las DB de FM24.
8. [`07-roadmap.md`](07-roadmap.md) — milestones, DoD y primeros pasos.
9. [`08-legal.md`](08-legal.md) — marcas, copyright, datos y checklist.
10. [`09-piramide-mexico.md`](09-piramide-mexico.md) — organigrama completo de México
    (Liga MX, Sub-21/Sub-19, Expansión, Femenil, Premier, TDP).
11. [`protocol/v0.md`](protocol/v0.md) — contrato JSON entre cliente y servidor.

## Reglas de la casa

- **Sin datos ni assets de SI.** El save de FM se importa solo si el usuario lo
  trae; nunca se redistribuye.
- **El servidor es la autoridad.** El cliente manda intents; no simula estado.
- **Todo dato tiene procedencia.** Fuente, licencia, método y confianza.
- **Todo cambio de estado es un comando auditable.** `command_id`, `seq`,
  journal y snapshot.
- **Nombres:** o reales con datos propios, o ficticios completos. Nada de
  “Cristian Rolando”.

## Relación con este repo (`fm-project`)

El repo actual tiene piezas probadas que puedes **estudiar y reutilizar**:

- `engine-rs/crates/fm-sim` — motor de partido en Rust.
- `engine-rs/crates/fm-state` — estado durable con journal/snapshots.
- `engine-rs/crates/fm-server` — WebSocket autoritativo.
- `fm_tui/` — TUI en Python/Textual (referencia de UX).
- `extract/` — herramientas para leer el save de FM (adaptador opcional).

La recomendación es un repo nuevo (`opengaffer`) con estos documentos y el
código que decidas portar, no seguir creciendo `fm-project`.
