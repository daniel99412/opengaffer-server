# 05 — Multijugador

## Modelo

- **Servidor autoritativo**: el estado vive en el servidor. El cliente solo
  manda intents y renderiza.
- **Mundo**: una partida con catálogo, reglas, semilla y ritmo.
- **Asiento (seat)**: un club dirigido por un humano o por la IA.
- **Invitación**: código de un solo uso o multi-uso, con TTL y rol.

## Ciclo de vida del mundo

```
lobby → running → paused → running → finished
```

- `lobby`: se crean asientos, se reclaman clubes, se espera el mínimo.
- `running`: avanza el calendario según el ritmo.
- `paused`: admin detiene; clientes siguen consultando.
- `finished`: temporada cerrada; se puede exportar.

## Crear y unirse

1. **Host** crea mundo: país, reglas, `min_managers`, `late_join`,
   `ai_policy`, `pace`, `seed`.
2. **Invita**: `session.invite` devuelve un código (ej. `GAFF-7K3M-9Q2P`).
3. **Amigo** entra: `session.join {code, display_name}`.
4. Ve clubes libres: `clubs.available`.
5. Reclama: `clubs.claim {club_uid, expected_version}`.
6. Recibe snapshot del mundo y briefing de su club.
7. Cuando se cumple `min_managers`, el admin hace `world.start`.

## Late join y control de un club de la IA

Dos políticas:

| política | cuándo | cómo |
|---|---|---|
| **cerrado** | mundos competitivos | solo se une en `lobby` |
| **abierto** | amigos/privados | se une en `running` y reclama un club IA |

Al reclamar un club IA:

1. El club pasa de `ai` a `human` en una transacción.
2. **Handover, no reset**: hereda tabla, finanzas, plantilla, lesiones,
   contratos y sanciones.
3. El servidor manda `manager.briefing`:
   - posición, puntos, forma;
   - presupuesto y deuda;
   - lesionados/sancionados;
   - últimos 5 resultados;
   - **decisiones de la IA** (fichajes, ventas, renovaciones).
4. El nuevo manager puede cambiar alineación/táctica antes del siguiente
   partido; el partido en curso no se interrumpe.

### IA caretaker

Mientras un club no tiene manager:

- Alinea y rota, pero **no vende** a sus mejores jugadores.
- No firma contratos largos ni cláusulas altas.
- Usa cantera si falta plantilla.
- No hace transferencias entre clubes IA (opcional por mundo).

Así el que llega no hereda un desastre.

## Ritmo

| modo | cómo avanza | cuándo usarlo |
|---|---|---|
| **asíncrono** | cuando todos están listos o vence el plazo | amigos, trabajo |
| **programado** | cada X horas (ej. 24 h) | ligas públicas |
| **en vivo** | todos conectados, host avanza | sesiones de fin de semana |

- El host tiene **lease de 30 s + epoch/fencing**: solo el host vigente avanza.
- Si el host se cae, otro puede tomar el lease (`host.takeover`) y continuar.
- Si un manager no aparece, sus órdenes guardadas o la IA juegan por él.

## Sincronización

- Cada cambio de estado produce un evento con `seq` monotónico.
- El cliente guarda `last_seq`.
- Al reconectar:
  - `sync.resume {last_seq}` → delta si hay journal;
  - si no, `snapshot {seq, state, checksum}` + delta posterior.
- El cliente descarta duplicados por `seq` y detecta huecos.
- Reintentos usan el mismo `command_id`: no duplican efectos.

## Consistencia

- **Idempotencia**: `commands(world_id, command_id)` único; se consulta antes
  de validar versión.
- **Concurrencia optimista**: `expected_version`; si no coincide, `conflict`
  con el estado actual.
- **Transacción única**: estado + journal + audit + command_log; commit antes
  de responder.
- **Un solo escritor**: actor por mundo + `flock`; SQLite en red prohibido.

## Matriz de fallas

| falla | detección | mitigación | recuperación |
|---|---|---|---|
| desconexión a media tx | timeout/heartbeat | tx atómica; no se aplica a medias | reconectar con `sync.resume` |
| entrega duplicada | `command_id` repetido | dedupe | devolver resultado original |
| fuera de orden | `seq` faltante | rechazar/esperar | pedir delta/snapshot |
| crash del servidor | reinicio | journal + snapshots | replay y continuar |
| crash del cliente | heartbeat | estado en servidor | reconectar |
| partición de red | ping/pong | backoff con jitter | reconectar |
| split brain | lease/epoch | fencing token | el perdedor se detiene |
| reloj desfasado | timestamps del servidor | no confiar en el cliente | usar reloj del servidor |
| upgrade de esquema | `schema_version` | migración + snapshot previo | restore si falla |
| journal corrupto | hash chain | detectar primer registro roto | snapshot + replay parcial |
| disco lleno | error de escritura | modo degradado | liberar espacio, reintentar |
| cliente lento | backpressure | límites y `sync.required` | resync |
| cliente malicioso | validación | rate limit, auth, sin confiar | ban/expulsar |

## Seguridad

- TLS (`wss`) en producción; token de sesión por asiento.
- Tokens hasheados en reposo, TTL y revocación.
- El cliente no recibe el save completo: solo su club y datos públicos.
- El servidor nunca ejecuta datos del cliente como código.
- Rate limit y tamaño máximo de mensaje.
- Audit de acciones sensibles (claim, kick, advance, admin).

## Pruebas obligatorias

- 2 clientes: uno avanza, el otro recibe el evento.
- Manager sin lease no puede avanzar.
- Reclamo simultáneo del mismo club: uno gana, otro `conflict`.
- Late join con club IA: hereda estado y recibe briefing.
- Host se cae: takeover y continúa sin duplicar.
- 100 reconexiones: mismo `state_digest`.
- 50 reenvíos del mismo `command_id`: un solo efecto.
- Crash con `.db` + `-wal`: recuperación correcta.
