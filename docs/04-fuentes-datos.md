# 04 — Fuentes de datos y licencias

Regla de oro: **si no sabes la licencia, no lo uses**. Cada dato debe poder
rastrearse hasta su fuente y su permiso.

## Fuentes recomendadas

| fuente | qué aporta | licencia / condición | uso |
|---|---|---|---|
| **Wikidata** | nombres, DOB, nacionalidad, clubes, IDs | CC0 | catálogo base |
| **Wikipedia** | carrera, palmarés | CC BY-SA (atribución) | validación; no copiar tablas masivas |
| **FBref / StatsBomb** | stats detalladas, xG | revisar términos; open data tiene licencia | métricas |
| **Understat** | xG de top 5 | scraping sujeto a términos | validación |
| **API-Football / Sportmonks** | fixtures, stats, alineaciones | comercial, licencia | producción |
| **football-data.org** | competiciones y resultados | tier gratis con límites | arranque |
| **OpenFootball (GitHub)** | datasets abiertos | revisar LICENSE por repo | arranque |
| **Transfermarkt** | valores de mercado | **prohibido scraping**; licencia comercial | solo validación si hay licencia |
| **Save de FM del usuario** | datos completos de su partida | propiedad del usuario; **no redistribuir** | adaptador privado |
| **Datos propios** | atributos y ratings | nuestros | núcleo |

## Lo que NO se usa

- Base de datos de SI/FM empaquetada o redistribuida.
- Escudos, kits, fotos o logos sin licencia.
- Scraping de sitios que lo prohíben.
- Nombres tipo “Cristian Rolando” / “Pionel Lessi”.

## Pipeline de catálogo

```
ingest → normalize → resolve IDs → compute ratings → validate → publish
```

1. **Ingest**: descarga/lee la fuente y guarda el crudo en `data/raw/<source>/`.
2. **Normalize**: nombres, fechas, países, posiciones a nuestro vocabulario.
3. **Resolve IDs**: `entity_external_ids` (Wikidata QID, API ID, FM uid).
4. **Compute**: métricas → rating (ver `03-rating-global.md`).
5. **Validate**: distribuciones, anclas, simulación.
6. **Publish**: data pack con versión y checksum.

## Data pack

```
catalog/
  opengaffer-2026.1/
    manifest.json         # versión, fecha, fuentes, licencias, checksum
    nations.json
    competitions.json
    clubs.json
    people.json
    attributes.json
    ratings.json
    rules.json
```

`manifest.json` mínimo:

```json
{
  "name": "opengaffer-2026.1",
  "schema_version": 1,
  "created_at": "2026-11-27T00:00:00Z",
  "sources": [
    {"name": "Wikidata", "license": "CC0", "url": "https://www.wikidata.org"}
  ],
  "checksum": "sha256:...",
  "notes": "México e Inglaterra; ratings propios"
}
```

## Procedencia por dato

Cada fila derivada guarda:

- `source_id`
- `method`: `measured | derived | estimated | user_supplied`
- `confidence`: `high | medium | low`
- `updated_at`
- evidencia (métrica, temporada, minutos) cuando aplique.

Esto te permite:
- Auditar de dónde salió un 17.
- Recalcular sin perder el origen.
- Responder si alguien pregunta por licencias.

## Adaptador de save de FM (opcional)

- Comando `gaffer-catalog import-fm --save <archivo.fm>`.
- Lee el save del usuario **en su máquina**; no lo sube ni lo redistribuye.
- Genera un data pack **privado** para ese mundo.
- Se documenta como “fuente: save del usuario”, no como dato de OpenGaffer.
- No es el camino por defecto para mundos públicos.

## Checklist legal de datos

- [ ] Cada fuente tiene licencia escrita en `data_sources`.
- [ ] Nada de assets de SI o de clubes sin licencia.
- [ ] Atribución donde la licencia la exige (Wikipedia, etc.).
- [ ] Data pack con checksum y manifiesto.
- [ ] El importador de FM no redistribuye nada.
- [ ] Registro de cambios de fuente y fecha.
