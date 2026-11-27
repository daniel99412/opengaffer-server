# 00 — Visión y alcance

## Qué es OpenGaffer

Un juego de gestión de fútbol **open source**, **multiplayer desde el diseño**,
donde un servidor autoritativo lleva el mundo y varias personas dirigen clubes
en la misma partida.

No es un clon de Football Manager ni usa su base de datos. Comparte el género y
la fantasía: dirigir un club, armar plantilla, jugar temporadas.

## Pilares

1. **Multiplayer real**: mundo persistente, invitaciones, reconexión exacta,
   varios managers en la misma liga.
2. **Datos propios**: catálogo con licencias y procedencia; ratings derivados
   de datos públicos, no copiados de SI.
3. **Motor determinista**: misma semilla + mismos comandos = mismo resultado.
   El servidor simula; el cliente solo muestra.
4. **Moddable**: data packs versionados (países, ligas, reglas, atributos).
5. **Abierto**: código, formato de datos y protocolo documentados.

## Personas

- **Commissioner / host**: crea el mundo, invita, decide ritmo y reglas.
- **Manager**: reclama un club, arma alineación, juega temporadas.

## Alcance de la v1

- **México completo** (ver `09-piramide-mexico.md`):
  - Liga MX, Liga MX Sub-21 y Sub-19 (calendario espejo),
  - Liga de Expansión MX,
  - Liga MX Femenil,
  - Liga Premier Serie A y Serie B,
  - Liga TDP por grupos.
- **Un mundo por servidor** (después multi-mundo).
- **2–8 managers**; la IA cubre el resto de clubes.
- **Temporada completa**: calendario, tabla, lesiones, tarjetas, ascenso/descenso.
- **Mercado básico**: fichajes y contratos.
- **Partido en vivo** con relato y cambios (sin 3D).
- **Reconexión y reanudación** exactas.

## No-objetivos de la v1

- 3D, animaciones, comentarios de voz.
- Base de datos mundial completa (200+ ligas).
- Draft, cantera profunda, ojeo avanzado.
- Apps móviles.
- Compatibilidad binaria con saves de FM.

## Criterios de éxito

- Dos personas juegan una temporada completa sin perder estado al desconectarse.
- El avance de un día con 20 partidos tarda menos de 1 segundo en servidor.
- Un manager nuevo puede reclamar un club a media temporada y entender su
  situación en menos de 2 minutos.
- El catálogo se puede reconstruir desde fuentes públicas con un comando.

## Nombre y marca

- Marca: **OpenGaffer**.
- Slug/repo: `opengaffer`.
- Binarios: `gaffer-server`, `gaffer-sim`.
- Lema: *“El fútbol se juega desde la banca.”*
- Disclaimer visible: no afiliado a Sports Interactive ni Sega.
