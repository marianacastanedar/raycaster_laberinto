# Créditos y Atribuciones

## Bibliotecas y Frameworks

### Raylib
- **Descripción**: Framework de desarrollo de videojuegos
- **Versión**: 6.0
- **Licencia**: Zlib License
- **URL**: https://www.raylib.com/
- **Uso**: Gestión de ventanas, gráficos, audio e input

### raylib-rs
- **Descripción**: Bindings de Rust para Raylib
- **Versión**: 6.0
- **Licencia**: Zlib License
- **URL**: https://github.com/deltaphc/raylib-rs
- **Uso**: Interfaz Rust para Raylib

### nalgebra-glm
- **Descripción**: Biblioteca de álgebra lineal para gráficos
- **Versión**: 0.18
- **Licencia**: Apache-2.0
- **URL**: https://github.com/dimforge/nalgebra
- **Uso**: Vectores 2D para posiciones y cálculos geométricos

### image
- **Descripción**: Biblioteca de procesamiento de imágenes
- **Versión**: 0.25
- **Licencia**: MIT/Apache-2.0
- **URL**: https://github.com/image-rs/image
- **Uso**: Carga de texturas PNG con soporte RGBA

## Recursos de Arte

### Texturas
Las texturas de paredes (`wall1.png`, `wall2.png`, `wall3.png`, `goal.png`) deben ser proporcionadas por el usuario según las especificaciones en `PREPARACION_LOCAL.md`.

**Especificaciones**:
- Formato: PNG
- Dimensiones: 64x64 píxeles
- Tipo: RGB o RGBA

**Fuentes sugeridas para texturas libres**:
- OpenGameArt.org
- Itch.io (sección de assets gratuitos)
- Kenney.nl (assets CC0)

### Sprites
El sprite de fuego (`fire.png`) debe ser proporcionado por el usuario.

**Especificaciones**:
- Formato: PNG con canal alpha
- Dimensiones recomendadas: 64x64 o similar
- Transparencia: Fondo transparente para composición correcta

### Audio

#### Música de fondo (bgm.ogg)
Debe ser proporcionada por el usuario en formato OGG Vorbis.

**Fuentes sugeridas para música libre**:
- OpenGameArt.org
- Incompetech.com (Kevin MacLeod, CC BY)
- FreePD.com (dominio público)

#### Efectos de sonido (footstep.ogg, victory.ogg)
Deben ser proporcionados por el usuario en formato OGG Vorbis.

**Fuentes sugeridas para efectos de sonido**:
- Freesound.org (requiere atribución según licencia)
- OpenGameArt.org
- Sonniss.com (paquetes gratuitos ocasionales)

## Fuente Bitmap

La fuente 8x8 utilizada para renderizar texto en las pantallas de UI fue implementada manualmente para este proyecto, basada en fuentes bitmap clásicas de sistemas de 8 bits.

## Algoritmos

### Raycasting
El algoritmo de raycasting está inspirado en las técnicas utilizadas en:
- **Wolfenstein 3D** (id Software, 1992)
- Tutorial de Lode Vandevenne: https://lodev.org/cgtutor/raycasting.html

### DDA (Digital Differential Analyzer)
Algoritmo clásico de gráficos por computadora para traversal de grids, utilizado para el raycasting eficiente.

## Desarrollo

Este proyecto fue desarrollado como parte del curso de Gráficos por Computadora utilizando:
- **Lenguaje**: Rust (edición 2021)
- **Editor**: Visual Studio Code con rust-analyzer
- **Control de versiones**: Git

## Agradecimientos

- Comunidad de Rust por las excelentes bibliotecas y herramientas
- Comunidad de Raylib por el framework accesible y bien documentado
- Lode Vandevenne por el tutorial de raycasting que sirvió de referencia
- id Software por pionerizar la técnica de raycasting en videojuegos

## Notas sobre Licencias

Este proyecto es de naturaleza educativa. Los usuarios son responsables de asegurar que todos los recursos de arte y audio utilizados cumplan con las licencias apropiadas para su uso específico.

Si distribuyes este proyecto con assets incluidos, asegúrate de:
1. Verificar las licencias de cada recurso
2. Incluir las atribuciones requeridas
3. Cumplir con los términos de uso de cada licencia

---

**Fecha de última actualización**: 2026-08-20
