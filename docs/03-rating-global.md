# 03 — Rating global y balance de stats

Objetivo: que un 15 en Liga MX, en la Premier o en la liga de Túnez **signifique
lo mismo en habilidad absoluta**, y que las distribuciones se sientan justas.

## Capas

### Capa 0 — Métricas crudas por 90 minutos

Por posición (ejemplos; define tu set):

| posición | métricas principales |
|---|---|
| GK | paradas %, PSxG-GA, salidas, pases completados, errores |
| CB | duelos aéreos %, intercepciones, despejes, pases progresivos |
| FB/WB | centros, conducciones progresivas, duelos, xA |
| DM | intercepciones, tackles, pases progresivos, faltas |
| CM | pases progresivos, xA, recuperaciones, conducciones |
| AM/W | xG+xA, ocasiones creadas, regates, presión |
| ST | goles, xG, tiros, duelos aéreos, asistencias |

Filtra por minutos (≥ 600 para confianza alta) y por temporada.

### Capa 1 — Normalización dentro de la liga

Para cada métrica `m`, posición `pos` y liga `L`:

```
z = (x_p90 - μ(L,pos,m)) / σ(L,pos,m)
```

Si `σ` es muy chico o la muestra es chica, usa el prior de la confederación.

### Capa 2 — Fuerza de liga

Cada competición tiene un coeficiente `S(L)`:

```
S(L) = 0.45*Elo_continental + 0.35*coeficiente_historico + 0.20*valor_mercado
```

- `Elo_continental`: resultados en copas continentales de sus clubes.
- `coeficiente_historico`: rendimiento de la liga en los últimos 5 años.
- `valor_mercado`: mediana de valores de la liga (señal débil, normalizada).
- Anclas: jugadores que se transfieren entre ligas. Si un jugador rinde `z=1.2`
  en L1 y `z=0.3` en L2, aprendes el delta entre ligas.

### Capa 3 — Habilidad absoluta (Bayes)

```
A = Σ w(pos,m) * z_m
A_hat = (n/(n+k)) * A + (k/(n+k)) * prior(age, pos, tier)
```

- `w`: pesos por posición (suman 1).
- `n`: minutos; `k` típico 900–1200.
- `prior`: nivel esperado por edad, posición y tier de liga.
- Opcional: ajuste por fuerza del rival y por contexto (local/visita).

### Capa 4 — Escala 1-20

```
ability = clamp(round(10 + 3.2 * A_hat + α*(S(L)-S_ref)), 1, 20)
```

- `S_ref`: liga de referencia (p. ej. Premier).
- `α`: 0.5–1.0 según calibración.
- Calibra con percentiles globales:

| percentil | ability |
|---|---|
| 99 | 19-20 |
| 90 | 16-17 |
| 75 | 14-15 |
| 50 | 11-12 |
| 25 | 9-10 |
| 10 | 7-8 |
| 1 | 4-6 |

## Potencial

No copies un PA. Modelo propio:

```
potential = ability + growth(age, minutes, trajectory, position)
```

- `growth`: curva por edad (pico 24–28), minutos jugados, progreso de 2 temporadas.
- Techo por posición y contexto (un GK madura más tarde).
- Marca `confidence`: sin minutos suficientes, el potencial es un rango.

## Cobertura global

| tier | ligas | datos | estrategia |
|---|---|---|---|
| A | Top 5, Brasil, Argentina, MX, MLS | ricos | modelo completo |
| B | resto Europa/Sudamérica | medios | modelo + prior de liga |
| C | Asia, África, divisiones bajas | escasos | prior por confederación + continentales + selección + minutos; `confidence: low` |

Para tier C:
- Muestra **rango** (`ability_range: [12,15]`) en vez de un número falso.
- Usa minutos y edad como señal mínima.
- No infles: si no hay datos, el prior manda.

## Validación (obligatoria)

1. **Distribuciones**: media y desviación por liga/posición; no todos 18.
2. **Transferencias**: predice el rendimiento del jugador en su nueva liga.
3. **Simulación**: tabla, goleadores y valores realistas por país.
4. **Continentales**: un club de liga débil no debe arrasar por ratings inflados.
5. **Casos ancla**: 20 jugadores conocidos por tier; revisa que el orden tenga
   sentido.
6. **Estabilidad**: recalcular con otra temporada no debe mover más de ±2 a la
   mayoría.

## Pitfalls

- **Inflación por liga fuerte**: no sumes el coeficiente dos veces (en el rating
  y en el motor). Decide: rating absoluto + motor que usa la fuerza de liga para
  el contexto del partido.
- **Muestras chicas**: 200 minutos no hacen un 18.
- **Sesgo de goles**: un delantero en un equipo que crea mucho no es mejor que
  uno que crea solo; usa xG y contexto.
- **Posiciones mal etiquetadas**: normaliza por posición real, no por la que
  dice el club.
- **Datos de mercado**: úsalos solo como validación, nunca como rating directo.

## Implementación (checklist)

- [ ] Define el set de métricas y sus pesos por posición.
- [ ] Construye el pipeline de normalización por liga/temporada.
- [ ] Calcula `S(L)` con al menos dos fuentes.
- [ ] Implementa el prior y el encogimiento bayesiano.
- [ ] Calibra contra percentiles y casos ancla.
- [ ] Guarda `model_version`, `confidence` y `minutes`.
- [ ] Escribe el reporte de validación por liga.
