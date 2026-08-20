mod caster;
mod framebuffer;
mod maze;
mod player;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use maze::Maze;
use player::Player;

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
pub const FOOTSTEP_INTERVAL: f32 = 0.45;

/// Colores del techo y el piso.
pub const CEILING_COLOR: u32 = 0x1A1A26;
pub const FLOOR_COLOR: u32 = 0x3B2E24;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycaster Laberinto")
        .build();

    // Carga el laberinto desde el archivo
    let maze = Maze::load("maze.txt", BLOCK_SIZE);

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

    // Ciclo principal
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // Actualiza el estado del jugador segun la entrada
        player.update(
            &rl,
            &maze,
            MOVE_SPEED,
            ROTATION_SPEED,
            PLAYER_RADIUS,
            BLOCK_SIZE,
            dt,
        );

        // Renderiza la escena en 3D usando raycasting
        let _zbuffer = caster::render_3d(
            &mut fb,
            &player,
            &maze,
            FOV,
            BLOCK_SIZE,
            CEILING_COLOR,
            FLOOR_COLOR,
        );

        // Actualiza la textura de raylib con el contenido del framebuffer
        texture
            .update_texture(&fb.buffer)
            .expect("no se pudo actualizar la textura");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Dibuja la textura en pantalla
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
