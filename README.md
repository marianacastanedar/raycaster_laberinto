# Raycaster Laberinto

Proyecto de gráficos por computadora que implementa un motor de raycasting 3D en Rust, inspirado en juegos clásicos como Wolfenstein 3D.

## Características

- **Raycasting 3D**: Motor de renderizado que proyecta un laberinto 2D en vista primera persona
- **Texturas**: Tres tipos de paredes texturizadas (piedra, ladrillo, madera)
- **Sprites billboards**: Llamas animadas con oclusión por z-buffer
- **Minimapa**: Vista cenital en tiempo real de la posición del jugador
- **Audio**: Música de fondo, efectos de pasos y sonido de victoria
- **Sistema de estados**: Pantallas de bienvenida, juego y éxito

## Controles

- **Flechas Arriba/Abajo**: Mover adelante/atrás
- **Flechas Izquierda/Derecha**: Rotar cámara
- **Mouse**: Mirar alrededor
- **Enter**: Comenzar juego / Reiniciar tras victoria
- **Escape**: Salir del juego

## Objetivo

Navega por el laberinto y llega a la meta (pared verde) evitando las llamas.

## Requisitos

- Rust 1.70 o superior
- Sistema operativo: Windows, Linux o macOS
- Dependencias (instaladas automáticamente por Cargo):
  - raylib 6.0
  - nalgebra-glm 0.18
  - image 0.25

## Estructura del proyecto

```
raycaster_laberinto/
├── src/
│   ├── main.rs          # Punto de entrada y loop principal
│   ├── audio.rs         # Sistema de audio (música y efectos)
│   ├── caster.rs        # Motor de raycasting
│   ├── framebuffer.rs   # Buffer de píxeles RGBA
│   ├── maze.rs          # Carga y gestión del laberinto
│   ├── minimap.rs       # Renderizado del minimapa
│   ├── player.rs        # Lógica del jugador y colisiones
│   ├── screens.rs       # Pantallas de UI y máquina de estados
│   ├── sprites.rs       # Renderizado de sprites billboards
│   └── textures.rs      # Carga de texturas
├── assets/
│   ├── textures/        # Texturas de paredes (64x64 PNG)
│   ├── sprites/         # Texturas de sprites (fire.png)
│   └── audio/           # Archivos de audio (OGG Vorbis)
├── maze.txt             # Definición del laberinto (25x17)
└── Cargo.toml           # Configuración del proyecto

```

## Compilación

```bash
cargo build --release
```

## Ejecución

```bash
cargo run --release
```

## Preparación de recursos

Asegúrate de tener los siguientes archivos en la carpeta `assets/`:

### Texturas (64x64 PNG)
- `assets/textures/wall1.png` - Pared de piedra
- `assets/textures/wall2.png` - Pared de ladrillo
- `assets/textures/wall3.png` - Pared de madera
- `assets/textures/goal.png` - Textura de la meta (verde)

### Sprites (PNG con transparencia)
- `assets/sprites/fire.png` - Sprite de llama

### Audio (OGG Vorbis)
- `assets/audio/bgm.ogg` - Música de fondo
- `assets/audio/footstep.ogg` - Efecto de pasos
- `assets/audio/victory.ogg` - Sonido de victoria

Consulta `PREPARACION_LOCAL.md` para instrucciones detalladas sobre cómo obtener estos recursos.

## Arquitectura técnica

### Raycasting
El motor utiliza el algoritmo DDA (Digital Differential Analyzer) para lanzar rayos desde la cámara y detectar intersecciones con las paredes. Se implementa corrección de efecto "ojo de pez" para evitar distorsión en los bordes de la pantalla.

### Sprites
Los sprites se renderizan como billboards (siempre mirando a la cámara) y utilizan un z-buffer para oclusión correcta con las paredes.

### Colisiones
Sistema de detección de colisiones por radio del jugador con resolución por eje para permitir deslizamiento suave a lo largo de las paredes.

### Texturas
Texturas de 64x64 píxeles con sombreado basado en distancia y orientación de la pared para dar sensación de profundidad.

## Convenciones de código

- **Identificadores**: En inglés
- **Comentarios**: En español
- **Nombres de archivos**: Sin tildes
- **Formato**: Estándar Rust (rustfmt)

## Licencia

Proyecto educativo desarrollado para el curso de Gráficos por Computadora.

Consulta `CREDITS.md` para atribuciones de recursos utilizados.
