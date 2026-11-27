# 09 — Pirámide de México (v1)

Alcance decidido: **todo el organigrama mexicano**, no solo dos ligas.

## Competencias objetivo

| competencia | nivel | equipos | formato | notas |
|---|---:|---:|---|---|
| **Liga MX** | 1 | 18 | Apertura/Clausura, 17 jornadas, liguilla 10 | reglas ya extraídas del save |
| **Liga MX Sub-21** | 1 ( juvenil) | 18 | espejo de Liga MX | mismo rival, misma localía |
| **Liga MX Sub-19** | 1 (juvenil) | 18 | espejo de Liga MX | mismo rival, misma localía |
| **Liga de Expansión MX** | 2 | 15–16 | Apertura/Clausura, 15 jornadas, liguilla 8 | reglas del save |
| **Liga MX Femenil** | 1 (femenil) | 18 | Apertura/Clausura + liguilla | **no está en FM24**; fuente externa |
| **Liga Premier Serie A** | 3 | 20 | grupos + liguilla 8 | en FM24: `Mexican Premier Division Serie A` |
| **Liga Premier Serie B** | 4 | 10 | doble, 18 jornadas, liguilla 4 | en FM24: `Serie B` |
| **Liga TDP** | 5 | 18+ grupos | regional, grupos | en FM24: `Mexican Third Division` |

> En FM24 también existen `Promotion League`, `Second Division`, `New Talents
> League` y `U20/U18/U23`. No los tomamos como verdad: la pirámide la defines
> tú; el save solo aporta formatos y clubes cuando coinciden.

## Decisión clave: club ≠ equipo

Un club (América) tiene **varios equipos**:

```
Club: América
├── Equipo: América (Liga MX)
├── Equipo: América Sub-21
├── Equipo: América Sub-19
└── Equipo: América Femenil
```

Por eso el modelo necesita `teams`, no solo `clubs`.

### Tablas

```
clubs(id, uuid, name, short_name, nation_id, ...)

teams(
  id, uuid, club_id, competition_id,
  name, gender, age_group, level,
  active, source, confidence
)

team_registrations(
  team_id, person_id, shirt_number,
  registered_from, registered_to,
  eligibility, source, confidence
)

competitions(
  id, uuid, name, short_name, nation_id,
  gender, age_group, tier, level,
  format, calendar_mode, mirror_of,
  parent_id, promotion_to, relegation_to,
  rules_json, source, confidence
)
```

`gender`: `male | female`.
`age_group`: `senior | u21 | u19 | u18 | ...`.
`calendar_mode`: `independent | mirror`.
`rules_json`: límites de plantilla, extranjeros, minutos de menores, etc.

Un jugador puede estar registrado en **senior y Sub-21** a la vez
(`team_registrations`), con reglas de elegibilidad y de no jugar dos partidos
el mismo día.

## Calendario espejo (Sub-21 y Sub-19)

1. Genera primero el calendario de **Liga MX**.
2. Para cada partido del senior, crea el partido del equipo juvenil:
   - mismo `round`, mismo `day` (o `day - 1` si quieres diferenciar);
   - mismo club local/visitante, resuelto a su equipo Sub-21/Sub-19;
   - `parent_fixture_id` apunta al partido senior.
3. Si un club no tiene equipo juvenil, el partido se marca `bye` o no se crea.
4. Liguilla: configurable (`mirror_playoffs: true/false`).
5. Regla de conflicto: un jugador registrado en ambos equipos **no puede**
   jugar dos partidos el mismo día; el servidor lo valida.

```
fixtures(
  id, competition_id, season_id, round, day,
  home_team_id, away_team_id,
  parent_fixture_id NULL,
  played, home_goals, away_goals, winner_team_id
)
```

## Ascenso y descenso

Grafo configurable, no hardcodeado:

```
Liga MX        ↔  Expansión MX      (0 plazas por regla real; configurable)
Expansión MX   ↔  Liga Premier A
Liga Premier A ↔  Liga Premier B
Liga Premier B ↔  Liga TDP
Liga MX Femenil: grafo propio (si existe)
```

Cada arista tiene:
- `slots` (cuántos suben/bajan),
- `playoff` (si el ascenso se decide por liguilla),
- `enabled` (la Liga MX real tiene 0; tú decides si lo activas).

## Reglas de plantilla (configurables)

- Tamaño de plantilla por competencia.
- **Sub-21 / Sub-19**: corte por año de nacimiento; plazas de mayores.
- **Regla de minutos de menores** (si la quieres): mínimo de minutos por torneo.
- Extranjeros por competencia.
- Canteranos/formados en casa.
- Femenil: plantilla y competición separadas; sin mezclar registros.

Todo en `rules_json` con versión, para no reprogramar el motor por cada torneo.

## Datos: qué tenemos y qué falta

| competencia | en FM24 | en nuestro extractor | fuente sugerida |
|---|---|---|---|
| Liga MX | sí (18) | `club_competitions` 18 | save / open data |
| Expansión | sí (15/16) | `club_competitions` 15 | save / open data |
| Premier A | sí (20, grupos) | no curada | open data / extraer `--all` |
| Premier B | sí (10) | no curada | open data / extraer `--all` |
| TDP | sí (18+ grupos) | no curada | open data / extraer `--all` |
| Femenil | **no** | no | sitio oficial / API / Wikipedia |
| Sub-21 / Sub-19 | no (hay U20/U18/U23) | no | definido por ti (espejo) |

Conclusión: **Liga MX + Expansión + Sub-21/Sub-19** se pueden arrancar ya con lo
que sabemos. Premier, TDP y Femenil requieren pipeline de datos externo.

## Escalabilidad

- TDP tiene cientos de clubes y 18+ grupos: el catálogo debe aguantar, pero el
  mundo puede simular solo las divisiones activas.
- Para divisiones sin datos suficientes: ratings por prior + `confidence: low`.
- No simules partido a partido las 5 divisiones si no hay managers ahí; usa
  simulación agregada o solo tabla.

## Orden de implementación recomendado

1. `clubs` + `teams` + `competitions` con `gender/age_group/tier`.
2. Liga MX senior con 18 clubes.
3. Sub-21 y Sub-19 espejo (18 + 18).
4. Expansión con 15/16.
5. Grafo de ascenso/descenso configurable.
6. Premier A/B.
7. TDP por grupos.
8. Femenil cuando tengas fuente de datos.

## Checklist para ti

- [ ] Agregar `teams` y `team_registrations`.
- [ ] Agregar `gender`, `age_group`, `calendar_mode`, `mirror_of`.
- [ ] Generar fixtures espejo con `parent_fixture_id`.
- [ ] Validar que un jugador no juegue dos partidos el mismo día.
- [ ] Modelar el grafo de ascenso/descenso como datos, no como `if`.
- [ ] Cargar Liga MX + Sub-21 + Sub-19 y probar una jornada.
- [ ] Definir `rules_json` de plantilla para cada competencia.
