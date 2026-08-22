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

/// Dimensiones de la ventana
pub const SCREEN_WIDTH: i32 = 1000;
pub const SCREEN_HEIGHT: i32 = 600;

/// Lado de una celda
pub const BLOCK_SIZE: f32 = 64.0;

/// Campo de vision en radianes (60 grados).
pub const FOV: f32 = std::f32::consts::PI / 3.0;

/// Velocidades
pub const MOVE_SPEED: f32 = 160.0;
pub const ROTATION_SPEED: f32 = 10.0;

/// Cuanto gira la camara con mouse
pub const MOUSE_SENSITIVITY: f32 = 0.003;

/// Radio de colision para que la camara se pegue a las paredes.
pub const PLAYER_RADIUS: f32 = 12.0;

/// Alto de la llama
pub const FIRE_SCALE: f32 = 0.55;

/// Segundos entre pasos mientras camina.
pub const FOOTSTEP_INTERVAL: f32 = 0.3;

pub const CEILING_COLOR: u32 = 0x1A1A26;
pub const FLOOR_COLOR: u32 = 0x3B2E24;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycaster Laberinto")
        .build();

    // Inicia audio
    let mut raudio =
        RaylibAudio::init_audio_device().expect("no se pudo inicializar el dispositivo de audio");

    // Carga el laberinto
    let maze = Maze::load("maze.txt", BLOCK_SIZE);

    // Carga todas las texturas de paredes
    let textures = TextureManager::load();

    let sprites = SpriteManager::load();

    // Carga las imagenes de pantallas
    let screen_images = ScreenImages::load();

    // Carga los recursos de audio
    let mut audio_manager = AudioManager::load(&mut raudio);

    // Crea el jugador
    let mut player = Player::new(maze.player_start);

    // Crea el framebuffer
    let mut fb = Framebuffer::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    fb.set_background_color(0x000000);

    // Crea una imagen de raylib
    let img = Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("no se pudo crear la textura desde la imagen");

    // Inicia la música de fondo
    audio_manager.start_music();

    // Estado inicial del juego
    let mut game_state = GameState::Welcome;

    // si gana
    let mut has_won = false;

    // Ciclo principal
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        audio_manager.update_music();

        // Maneja estado
        match game_state {
            GameState::Welcome => {
                render_welcome(&mut fb, &screen_images);
                // Enter para comenzar
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_state = GameState::Playing;
                    rl.disable_cursor(); 
                }
            }
            GameState::Playing => {
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

                // si el jugador alcanzó la meta
                if !has_won && player.has_reached_goal(&maze, BLOCK_SIZE) {
                    has_won = true;
                    audio_manager.play_victory();
                    game_state = GameState::Success;
                    rl.enable_cursor();
                }

                // Renderiza la escena con raycasting
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

                sprites.render_sprites(
                    &mut fb,
                    &player,
                    &maze.fire_positions,
                    &zbuffer,
                    FOV,
                    BLOCK_SIZE,
                    FIRE_SCALE,
                );

                minimap::render_minimap(&mut fb, &maze, &player, BLOCK_SIZE);
            }
            GameState::Success => {
                render_success(&mut fb, &screen_images);

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    player = Player::new(maze.player_start);
                    has_won = false;
                    audio_manager.reset_victory();
                    game_state = GameState::Playing;
                    rl.disable_cursor();
                }
            }
        }

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
