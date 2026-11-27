# 06 — Aprendizajes de las DB de FM24

Volcado técnico de lo que aprendimos leyendo el save y la instalación de FM24.
Sirve para **entender el dominio** y diseñar nuestro modelo, **no** para copiar
la base de SI.

> Estados de confianza: **alta** = verificado con datos/partidos; **media** =
> evidencia fuerte sin cierre total; **baja** = hipótesis.

## 1. Contenedores

| fuente | qué es | notas |
|---|---|---|
| `*.fm` | save del usuario | contenedor con índice zstd; `DATA_BASE = 26` |
| `game_db.dat` | estado del save | calendario, etapas, personas |
| `client_db.dat` | base de instalación (editable) | datos de referencia |
| `server_db.dat` | misma base con campos extra | no es otra entidad |
| `lang_db.dat` / `lang_update_db.dat` | nombres localizados | el update es delta |
| `comp_<uid>.dat` | reglas por competición | cabecera 51 B + bloque tagged |
| `fix_man.dat` | calendario de partidos | 68 B por registro |
| `comp_man.dat` | reglas de fechas por etapa | sin clubes ni uid de comp |

## 2. Inventario (`db_tables`)

- **108 entradas** client/server; **102 extraídas**, 6 vacías.
- **84,259 clubes**, **62,517 jugadores**, **4,008 staff**, **7,047
  competiciones**, **251 naciones**, **173 monedas**, **48,413 estadios**,
  **56,293 ciudades**.
- Pools: **245,981** nombres, **542,060** apellidos, **85,668** comunes.
- Extraído después: **792,149** historial, **616,665** personas,
  **873,709** nombres extra, **10,075** registros de competición,
  **9,261** club_links, **6,535** nombres inline.

## 3. Tablas de catálogo decodificadas

| tabla | n | qué guarda | layout clave | conf. |
|---|---:|---|---|---|
| `table_2` ciudades | 56,293 | ciudad, país, clima, región | 43 B: lat +3, lon +7, nación +12, clima +16, altitud +20, región +27, index +32, uid +36 | alta |
| `table_3` clubes (client) | 84,259 | club, nación, ciudad | variable; nación +9 o +13 si `0xFFFFFFFF`; ciudad +25; nombre inline | alta |
| `table_3` clubes (server) | 84,259 | club + ~200 B extra | prefijo igual; extras sin nombrar | media |
| `table_extra_3` afición | 84,259 | 6 valores 1-20 | bloque antes de `00*6 03 FF*8 0A*6` | alta |
| `table_21` estadios | 48,413 | estadio, ciudad, coords | client 38 B; server 181 B (capacidad, año) | alta |
| `table_25` competiciones | 7,047 | nombre, país, continente, tier, tipo | 127/128 B; nación end-70, continente end-74, tier end-111, tipo end-108 | alta |
| `table_24` club→estadio | 84,507 | enlace | estadio en uid_pos+25 | alta |
| `table_45` leyendas | 84,477 | leyendas por club | sección con nombre inline | media |
| `table_12` premios | 28,597 | historial de premios | — | media |
| `table_48` premios 2 | 5,210 | ganador + año | — | media |
| `table_47` regiones | 2,488 | región local | nación en uid_pos+9 | media |
| `table_27` clima | 334 | perfil climático | `[u16][u32 len][name][106 B]` | alta |
| `table_41` monedas | 173 | moneda + tipo de cambio | — | alta |
| `table_0` premios | 3,545 | catálogo | 77 B | media |
| `table_18` derbis | 1,567 | rivalidades | — | media |
| `name_table_0/1/2` | — | **metadata**: idioma/frecuencia | idioma `+6 & 0x7F`; frecuencia `+8` | alta |
| `name_table_extra_0/1/2` | 873,709 | **cadenas reales** | — | alta |

## 4. Save / runtime

### `comp_*.dat`

- Cabecera FM24 de **51 B**: magic `\x03\x01.dat`, constante `f2 01`, flags,
  `season_year` en +20, `f32 1.0` en +26, `base_year 2000` en +45,
  `n_fields` en **+47**.
- Bloque **tagged**: `[tag 4 B al revés][0x01][tipo][valor]`. Los elementos de
  lista van sin tag.
- Tipos: `0x0a` contenedor, `0x0b` lista, `0x11` u8, `0x12` u16,
  `0x01/0x15` u32, `0x02` fourcc, `0x0f` par u32, `0x05` f64,
  `0x13/0x14/0x18` u64 (**0x13 mide 8 B en FM24**, no 4), `0x19` f32,
  `0x1a` string, `0x20` timestamp.
- 1,108 competiciones parsean; 1,073 estrictas.

### `fix_man.dat`

- Registro de **68 B**:
  - `-12`: marcador `0x18`
  - `-11`: `stage_id`
  - `-7`: ordinal de estadio + 1
  - `0`: `home_team_id`; `+4`: `0xFF`; `+7`: `away_team_id`; `+11`: `0xFF`
  - `+13`: fecha (`u16 día_del_año | slot<<9`, `u16 año`)
  - `+26`: temporada

### `game_db.dat` etapas

- Fila de **33 B**: `prev_stage_id | stage_id | stage_id | 0 | comp_index |
  grupo | ronda | ? | ?`.
- `comp_index` es de instalación, **no** el uid.

### Staff

- `habilidad = header + 21 + 3*n` (n = byte `header+13`); bloque de atributos en
  `habilidad + 36`.
- 24 atributos: 20 en el bloque, más cantera `header+43` (1-20), disciplina
  `header+35` (1-20), determinación `header+44` ((raw+2)//5) y adaptabilidad
  `personality[0]`.
- Puesto de referencia `r4` en `header+25` (34.5 % trae 0); `kind=3` (newgens)
  ignora el desplazamiento `3n`.
- Slot 18 = “Analizando datos”; slot 22 desconocido.

### Personas e historial

- **616,665** filas de persona con cabecera `[índice][uid][uid]` y fechas.
- **792,149** filas de historial: año +4, club +10, competición +14
  (97.5 % / 96.7 % resuelven a clubes/comps reales).
- **10,075** registros de competición: comp en +10, años en +20/+22.
- **6,535** nombres inline; `year` = nacimiento (validado).

## 5. Semántica de dominio aprendida

- **Reglas reales** por competición: formato, equipos, jornadas, puntos,
  liguilla, descensos, premios y TV (1,108 filas).
- **Plantillas reales** vía calendario + etapas: `club_competitions` (191 filas
  curadas: Liga MX 18, PL 20, Championship 24, Expansión 15/16…).
- **Puestos de staff** FM24: 26 preparador físico, 34 porteros, 64 jefe de
  cantera, 22 balón parado, 10 director de fútbol, 20 segundo entrenador…
- **Atributos de staff**: 24, escala `(raw+2)//5`, más 4 externos.
- **Valuación centinela**: `300,000,000` significa “sin valor”, no 300 M.
- **Puestos**: `r4` es referencia, no siempre el contractual.
- **Client vs server**: mismo dominio, el server agrega detalle.

## 6. Trampas y patrones

| trampa | lección |
|---|---|
| `club_index ≠ uid` | guarda ambos y mapea; los uid cambian entre saves |
| `0x13` mide 4 vs 8 | verifica tamaño por tipo; FM24 usa 8 |
| desplazamiento `3n` en staff | el layout depende del número de roles |
| sentinelas (`300M`, `0xFFFFFFFF`) | normaliza a `NULL` + confianza |
| tablas duplicadas client/server | no son entidades distintas |
| frames mal etiquetados | el trailer directory manda |
| nombres en `_extra`, metadata en `name_table_*` | no confundas |
| variables sin layout | guarda crudo + documenta; no inventes |
| datos de runtime vs editor | sepáralos siempre |

## 7. Qué copiar conceptualmente

1. **Catálogo vs estado**: referencia inmutable + mundo mutable.
2. **Índice interno + UID estable**: nosotros UUID + mapeo externo.
3. **Versionado de catálogo/reglas** por mundo.
4. **Procedencia por dato**.
5. **Tablas de enlace** (club↔estadio, club↔competición, club↔club).
6. **Runtime separado del editor**.

## 8. Qué NO copiar

- **CA/PA y atributos ocultos** como verdad revelada.
- **Selección y valores** de atributos de SI.
- **Nombres de campos/enums** idénticos.
- **Un esquema gigante** sin separación.
- **Dependencia del save de FM** como núcleo.

## 9. Referencia de herramientas del repo actual

- `extract/build_db.py` — construye `fm.db` desde el save.
- `extract/comp_rules.py` — reglas de competición.
- `extract/club_competitions.py` — club → división desde el calendario.
- `extract/staff_attrs.py` — atributos y rating de staff.
- `extract/import_pending.py` — extracción masiva + crudo.
- `docs/REGLAS_COMPETENCIAS.md`, `docs/STAFF_ATTRS.md`,
  `docs/TABLAS_PENDIENTES.md` — detalle.

> Si algún día haces un adaptador de save, estas herramientas son la referencia.
> No las metas al núcleo de OpenGaffer.
