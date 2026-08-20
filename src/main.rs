mod audio;
mod caster;
mod framebuffer;
mod maze;
mod minimap;
mod player;
mod screens;
mod sprites;
mod textures;

use raylib::prelude::*;

use audio::AudioManager;
use framebuffer::Framebuffer;
use maze::Maze;
use player::Player;
use screens::{render_success, render_welcome, GameState, ScreenImages};
use sprites::SpriteManager;
use textures::TextureManager;

/// Dimensiones de la ventana en pixeles.
pub const SCREEN_WIDTH: i32 = 1000;
pub const SCREEN_HEIGHT: i32 = 600;

/// Lado de una celda en coordenadas de mundo, en pixeles.
/// Coincide con el ancho de las texturas (64) para que el muestreo sea 1:1.
pub const BLOCK_SIZE: f32 = 64.0;

/// Campo de vision, en radianes (60 grados).
pub const FOV: f32 = std::f32::consts::PI / 3.0;

/// Velocidades en unidades por segundo; se multiplican por delta time.
pub const MOVE_SPEED: f32 = 160.0;
pub const ROTATION_SPEED: f32 = 2.5;

/// Cuanto gira la camara por pixel de movimiento del mouse.
pub const MOUSE_SENSITIVITY: f32 = 0.003;

/// Radio de colision del jugador. Evita que la camara se pegue a las paredes.
pub const PLAYER_RADIUS: f32 = 12.0;

/// Alto de la llama respecto al alto de una pared a la misma distancia.
pub const FIRE_SCALE: f32 = 0.55;

/// Segundos entre pasos mientras el jugador camina.
pub const FOOTSTEP_INTERVAL: f32 = 0.3;

/// Colores del techo y el piso.
pub const CEILING_COLOR: u32 = 0x1A1A26;
pub const FLOOR_COLOR: u32 = 0x3B2E24;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycaster Laberinto")
        .build();

    // Inicializa el dispositivo de audio
    let mut raudio =
        RaylibAudio::init_audio_device().expect("no se pudo inicializar el dispositivo de audio");

    // Carga el laberinto desde el archivo
    let maze = Maze::load("maze.txt", BLOCK_SIZE);

    // Carga todas las texturas de paredes
    let textures = TextureManager::load();

    // Carga la textura de sprites (fuego)
    let sprites = SpriteManager::load();

    // Carga las imagenes de las pantallas de bienvenida y éxito
    let screen_images = ScreenImages::load();

    // Carga los recursos de audio
    let mut audio_manager = AudioManager::load(&mut raudio);

    // Crea el jugador en la posicion inicial del laberinto
    let mut player = Player::new(maze.player_start);

    // Crea el framebuffer que usaremos para renderizar pixel por pixel
    let mut fb = Framebuffer::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    fb.set_background_color(0x000000); // Negro

    // Crea una imagen de raylib con el tamano de la pantalla
    let img = Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);

    // Convierte la imagen a una textura que se puede dibujar
    let mut texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("no se pudo crear la textura desde la imagen");

    // Inicia la música de fondo
    audio_manager.start_music();

    // Estado inicial del juego
    let mut game_state = GameState::Welcome;

    // Variable para rastrear si el jugador ya ganó
    let mut has_won = false;

    // Ciclo principal
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // Actualiza la música (IMPORTANTE: debe llamarse cada frame)
        audio_manager.update_music();

        // Maneja las transiciones de estado
        match game_state {
            GameState::Welcome => {
                // Renderiza la pantalla de bienvenida
                render_welcome(&mut fb, &screen_images);

                // Espera que el jugador presione Enter para comenzar
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_state = GameState::Playing;
                    rl.disable_cursor(); // Oculta el cursor al comenzar a jugar
                }
            }
            GameState::Playing => {
                // Actualiza el estado del jugador segun la entrada
                let is_moving = player.update(
                    &rl,
                    &maze,
                    MOVE_SPEED,
                    ROTATION_SPEED,
                    MOUSE_SENSITIVITY,
                    PLAYER_RADIUS,
                    BLOCK_SIZE,
                    dt,
                );

                // Actualiza el sistema de pasos
                audio_manager.update_footsteps(is_moving, FOOTSTEP_INTERVAL, dt);

                // Verifica si el jugador alcanzó la meta
                if !has_won && player.has_reached_goal(&maze, BLOCK_SIZE) {
                    has_won = true;
                    audio_manager.play_victory();
                    game_state = GameState::Success;
                    rl.enable_cursor(); // Muestra el cursor en la pantalla de éxito
                }

                // Renderiza la escena en 3D usando raycasting
                let zbuffer = caster::render_3d(
                    &mut fb,
                    &player,
                    &maze,
                    &textures,
                    FOV,
                    BLOCK_SIZE,
                    CEILING_COLOR,
                    FLOOR_COLOR,
                );

                // Renderiza los sprites de fuego usando el z-buffer
                sprites.render_sprites(
                    &mut fb,
                    &player,
                    &maze.fire_positions,
                    &zbuffer,
                    FOV,
                    BLOCK_SIZE,
                    FIRE_SCALE,
                );

                // Renderiza el minimapa en la esquina superior derecha
                minimap::render_minimap(&mut fb, &maze, &player, BLOCK_SIZE);
            }
            GameState::Success => {
                // Renderiza la pantalla de éxito
                render_success(&mut fb, &screen_images);

                // Espera que el jugador presione Enter para reiniciar
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Reinicia el estado del juego
                    player = Player::new(maze.player_start);
                    has_won = false;
                    audio_manager.reset_victory();
                    game_state = GameState::Playing;
                    rl.disable_cursor();
                }
            }
        }

        // Actualiza la textura de raylib con el contenido del framebuffer
        texture
            .update_texture(&fb.buffer)
            .expect("no se pudo actualizar la textura");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Dibuja la textura en pantalla
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        // Dibuja el contador de FPS solo durante el juego
        if game_state == GameState::Playing {
            d.draw_fps(10, 10);
        }
    }
}
