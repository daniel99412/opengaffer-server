# 08 — Legal y propiedad intelectual

> Esto no es asesoría legal. Para un lanzamiento comercial, consulta a un
> abogado de propiedad intelectual (1 hora bien invertida).

## Marca

- **Football Manager**, **FM**, **Sports Interactive** y sus logos son marcas de
  Sports Interactive/Sega.
- **OpenGaffer** no usa “FM” en la marca. Aun así, evita cualquier sugerencia de
  afiliación.
- Disclaimer obligatorio en README y sitio:
  > OpenGaffer no está afiliado, patrocinado ni respaldado por Sports
  > Interactive ni Sega. Football Manager es marca de sus respectivos dueños.
- No uses logos de SI ni de clubes sin licencia.
- Registra la marca si el proyecto crece (IMPI en México, clases 9 y 41).

## Copyright y base de datos

- Los **nombres y hechos** (jugador, edad, nacionalidad, club) no son obra.
- La **compilación** de una base de datos puede estar protegida (derecho sui
  generis en la UE; competencia desleal en México).
- **No copies** la selección, estructura creativa, valores de atributos ni
  textos de FM.
- Un esquema relacional es funcional; no es obra. Aun así, diseña el tuyo y
  documenta que es independiente.

## Derecho de imagen / publicidad

- Usar el nombre de una persona en contexto informativo suele ser aceptable.
- Usar su imagen, apodo o un parecido evidente en un producto comercial es
  riesgoso.
- **Nada de** “Cristian Rolando” / “Pionel Lessi”.
- Sin fotos de jugadores sin licencia.

## Clubes y competiciones

- Nombre y ciudad como texto: generalmente informativo.
- Escudos, kits, logos y mascotas: marca registrada; no sin licencia.
- Precedente: Manchester United demandó a Sega por uso de nombre/logo en FM.

## Datos de terceros

- Cada fuente debe tener licencia y atribución.
- Prohibido scraping donde los términos lo vedan (p. ej. Transfermarkt).
- Guarda evidencia: URL, licencia, fecha, método.

## Save de FM del usuario

- Es del usuario. OpenGaffer puede leerlo **en su máquina** para un mundo
  privado.
- No se sube, no se redistribuye, no se empaqueta.
- Se documenta como “fuente: save del usuario”.

## Checklist antes de publicar

- [ ] Sin DB de SI ni valores copiados.
- [ ] Sin logos/escudos/kits/fotos sin licencia.
- [ ] Sin nombres parodia.
- [ ] Disclaimer de no afiliación visible.
- [ ] `TRADEMARKS.md` y `LICENSE` (código) + licencia de datos.
- [ ] Fuentes con licencia y atribución.
- [ ] Aviso de privacidad si hay cuentas (LFPDPPP en México).
- [ ] Revisión legal para launch comercial.
- [ ] Plan de respuesta a un C&D: rename y remover assets rápido.

## Texto sugerido para `TRADEMARKS.md`

```
OpenGaffer es un proyecto independiente y open source. No está afiliado,
patrocinado ni respaldado por Sports Interactive Limited ni Sega Corporation.
"Football Manager", "FM", "Sports Interactive" y sus logos son marcas de sus
respectivos dueños. OpenGaffer no distribuye bases de datos, imágenes, escudos
ni assets de esos productos. La compatibilidad con saves de Football Manager es
solo de importación local por parte del usuario.
```
