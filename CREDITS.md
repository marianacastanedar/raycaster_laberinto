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

## Recursos

### Texturas
Las texturas de paredes (`wall1.png`, `wall2.png`, `wall3.png`, `goal.png`) proporcionadas por la página: Kenney or www.kenney.nl.

**Especificaciones**:
- Formato: PNG
- Dimensiones: 64x64 píxeles
- Tipo: RGB o RGBA

## Fuente Bitmap

La fuente 8x8 utilizada para renderizar texto en las pantallas de UI fue hecha manualmente para este proyecto, basada en fuentes bitmap clásicas de sistemas de 8 bits.

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
