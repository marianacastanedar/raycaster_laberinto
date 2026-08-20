use raylib::prelude::*;

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

    // Ciclo principal
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
