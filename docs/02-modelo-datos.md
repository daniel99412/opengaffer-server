# 02 — Modelo de datos

## Principios

1. **Catálogo vs mundo**: lo que no cambia (referencia) se separa de lo que
   cambia (partida).
2. **UUID + IDs externos**: cada entidad tiene un UUID estable propio y un
   mapeo a fuentes (Wikidata, API, save de FM).
3. **Procedencia**: cada dato derivado guarda fuente, licencia, método,
   confianza y fecha.
4. **Versionado**: catálogo, reglas, motor y esquema tienen versión; un mundo
   queda pinneado.
5. **Sin CA/PA**: usamos `ability`, `potential` y `confidence` propios.
6. **Nada de campos creativos de SI**: nuestra taxonomía de atributos.

## Catálogo (referencia, versionado)

### Geografía y competiciones

```
nations(id, uuid, name, code, confederation_id, created_at)
confederations(id, uuid, name, code)
regions(id, uuid, nation_id, name)
competitions(id, uuid, nation_id, name, short_name, tier, type,
             format, parent_id, logo_asset_id NULL)
competition_rules(competition_id, version, teams, rounds, points_win,
                  points_draw, relegation_slots, promotion_slots,
                  playoff_teams, split, prize_pool, tv_money, source,
                  confidence, updated_at)
```

### Clubes y personas

```
clubs(id, uuid, nation_id, region_id, city, name, short_name,
      founding_year, stadium_id NULL, reputation, source, confidence)
stadiums(id, uuid, name, city, capacity, built_year, lat, lon)
people(id, uuid, kind, first_name, last_name, common_name, birth_date,
       nation_id, height_cm, foot, source, confidence)
staff_roles(id, code, name, group)          -- catálogo propio
player_positions(person_id, position, familiarity)  -- 1-20 o 0-1
```

`people.kind`: `player | staff | referee | agent`.

### Atributos (nuestra taxonomía)

```
attribute_defs(id, code, name, group, kind, scale_min, scale_max,
               description)
  -- groups: technical, mental, physical, goalkeeping, knowledge
player_attributes(person_id, attribute_id, value, source, method,
                  confidence, updated_at)
```

Reglas:
- `value` en escala 1-20 (o la que definas), nunca 0 como “desconocido”:
  usa `NULL` + `confidence`.
- `method`: `measured | derived | estimated | user_supplied`.
- `confidence`: `high | medium | low`.

### Ratings derivados

```
rating_models(id, version, description, created_at, params_json)
player_ratings(person_id, model_id, ability, potential, confidence,
               minutes, computed_at)
club_ratings(club_id, model_id, attack, midfield, defense, overall,
             computed_at)
```

El rating **no es un dato de entrada**: se recalcula y se versiona.

### Contratos

```
contracts(id, person_id, club_id, start_date, end_date, wage, bonus,
          release_clause, squad_role, source, confidence)
```

### Procedencia y versiones

```
data_sources(id, name, url, license, license_url, terms_notes)
dataset_versions(id, version, created_at, checksum, notes)
entity_external_ids(entity_type, entity_id, source_id, external_id)
attribute_evidence(id, person_id, attribute_id, source_id, raw_value,
                   metric, season, minutes, notes)
```

## Mundo (estado mutable)

```
worlds(id, uuid, name, status, catalog_version, ruleset_version,
       engine_version, schema_version, seed, pace, min_managers,
       late_join, ai_policy, public, created_at)
world_config(world_id, key, value)
seats(world_id, club_id, manager_id NULL, status, claimed_at, version)
accounts(id, uuid, email, display_name, password_hash, created_at)
managers(id, uuid, account_id, world_id, display_name, reputation)
invites(id, uuid, world_id, code_hash, role, expires_at, max_uses, uses)
seasons(world_id, id, label, start_date, end_date, status)
fixtures(id, world_id, season_id, competition_id, round, day,
         home_club_id, away_club_id, played, home_goals, away_goals,
         winner_club_id NULL, neutral)
standings(world_id, season_id, competition_id, club_id, played, won,
          drawn, lost, gf, ga, points)
match_events(id, fixture_id, minute, kind, side, person_id NULL, data_json)
injuries(id, world_id, person_id, kind, days_left, started_at)
suspensions(id, world_id, person_id, competition_id, matches_left, reason)
transfers(id, world_id, person_id, from_club_id, to_club_id, fee,
          wage, date, status)
finances(world_id, club_id, balance, transfer_budget, wage_budget,
         updated_at)
finance_log(id, world_id, club_id, day, amount, concept, ref)
```

### Operación / multiplayer

```
commands(world_id, command_id, actor_id, seq, cmd, params_hash,
         result_json, created_at, PRIMARY KEY(world_id, command_id))
journal(world_id, seq, event_json, prev_hash, hash, created_at)
snapshots(world_id, seq, state_digest, blob_zstd, created_at)
audit_log(world_id, id, actor_id, command_id, cmd, target, before_hash,
          after_hash, result, created_at, prev_hash, hash)
sessions(world_id, id, account_id, token_hash, expires_at, last_seq)
```

## Claves y constraints

- **UUID** para identidad pública; `id` entero para joins internos.
- `seats`: `UNIQUE(world_id, club_id)`; un club, un manager.
- `commands`: PK `(world_id, command_id)` para idempotencia.
- `journal`: PK `(world_id, seq)`; `prev_hash` encadenado.
- `audit_log`: `UNIQUE(world_id, command_id)`; append-only.
- `invites`: `code_hash` único, TTL, usos máximos.
- `player_attributes`: PK `(person_id, attribute_id)`.
- `entity_external_ids`: `UNIQUE(source_id, entity_type, external_id)`.

## Versionado y migraciones

- `schema_meta(schema_version)` en cada DB.
- Migraciones numeradas, idempotentes, con `up` explícito.
- Si el save es más nuevo que el binario: rechazar con mensaje claro.
- `catalog_version` se guarda como string + checksum; el mundo no se actualiza
  solo.
- Compatibilidad: un mundo viejo se puede migrar con un comando explícito y
  snapshot previo.

## JSON vs columnas

- Columnas: lo que se filtra/ordena (tabla, fixtures, finanzas, ratings).
- JSON: atributos variables, eventos, params de comandos, datos de proveedor.
- Nunca metas en JSON algo que necesites indexar con frecuencia.

## Ejemplo mínimo de DDL (ilustrativo)

```sql
CREATE TABLE clubs (
  id            INTEGER PRIMARY KEY,
  uuid          TEXT NOT NULL UNIQUE,
  nation_id     INTEGER NOT NULL REFERENCES nations(id),
  name          TEXT NOT NULL,
  short_name    TEXT,
  reputation    INTEGER,
  source_id     INTEGER REFERENCES data_sources(id),
  confidence    TEXT CHECK (confidence IN ('high','medium','low'))
);

CREATE TABLE player_attributes (
  person_id     INTEGER NOT NULL REFERENCES people(id),
  attribute_id  INTEGER NOT NULL REFERENCES attribute_defs(id),
  value         INTEGER CHECK (value BETWEEN 1 AND 20),
  source_id     INTEGER REFERENCES data_sources(id),
  method        TEXT CHECK (method IN ('measured','derived','estimated','user')),
  confidence    TEXT CHECK (confidence IN ('high','medium','low')),
  updated_at    TEXT NOT NULL,
  PRIMARY KEY (person_id, attribute_id)
);
```

## Checklist para ti

- [ ] Define la taxonomía de atributos (códigos, grupos, escala).
- [ ] Escribe las migraciones del catálogo y del mundo.
- [ ] Implementa el mapeo `entity_external_ids`.
- [ ] Decide qué tablas van en la DB del mundo y cuáles en el catálogo.
- [ ] Agrega constraints de unicidad e índices de consulta.
- [ ] Documenta el formato de data pack y su checksum.
