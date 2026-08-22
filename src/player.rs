use nalgebra_glm::Vec2;
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;

/// estado del jugador: posicion y orientacion.
pub struct Player {
    /// Posicion
    pub pos: Vec2,
    /// Angulo
    pub a: f32,
}

impl Player {
    /// Crea un nuevo jugador en la posicion especificada.
    pub fn new(pos: Vec2) -> Self {
        Self { pos, a: 0.0 }
    }

    /// Maneja movimiento (flechas arriba/abajo), rotacion (flechas izq/der y mouse) y colisiones.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        rl: &RaylibHandle,
        maze: &Maze,
        move_speed: f32,
        rotation_speed: f32,
        mouse_sensitivity: f32,
        player_radius: f32,
        block_size: f32,
        dt: f32,
    ) -> bool {
        // Rotacion con flechas izquierda y derecha
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.a += rotation_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.a -= rotation_speed * dt;
        }

        // Rotacion con mouse (solo eje horizontal)
        let mouse_delta = rl.get_mouse_delta();
        self.a += mouse_delta.x * mouse_sensitivity;

        // Normaliza el angulo al rango [0, 2π)
        self.a = self.a.rem_euclid(std::f32::consts::TAU);

        // Calcula el vector de direccion de vista
        let dir_x = self.a.cos();
        let dir_y = self.a.sin();

        // Movimiento adelante y atras con up y down
        let mut move_dir = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            move_dir += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            move_dir -= 1.0;
        }

        if move_dir != 0.0 {
            let speed = move_speed * dt;
            let new_x = self.pos.x + dir_x * speed * move_dir;
            let new_y = self.pos.y + dir_y * speed * move_dir;

            // Resuelve colisiones
            let test_pos_x = Vec2::new(new_x, self.pos.y);
            if !self.check_collision(test_pos_x, maze, player_radius, block_size) {
                self.pos.x = new_x;
            }
            let test_pos_y = Vec2::new(self.pos.x, new_y);
            if !self.check_collision(test_pos_y, maze, player_radius, block_size) {
                self.pos.y = new_y;
            }
            true
        } else {
            false
        }
    }

    /// Verifica si colisiona con las paredes del laberinto.
    /// Usa el radio para evitar que la camara se pegue a las paredes.
    fn check_collision(&self, pos: Vec2, maze: &Maze, radius: f32, block_size: f32) -> bool {
        let directions = [
            Vec2::new(radius, 0.0),
            Vec2::new(-radius, 0.0),
            Vec2::new(0.0, radius),
            Vec2::new(0.0, -radius),
        ];

        for dir in &directions {
            let check_pos = pos + dir;
            let cell_x = (check_pos.x / block_size) as usize;
            let cell_y = (check_pos.y / block_size) as usize;

            // Si es solida, hay colision
            if maze.is_solid(cell_x, cell_y) {
                return true;
            }
        }

        false
    }

    /// si el jugador ha alcanzado la meta.
    /// true si alguna de las celdas vecinas es 'g'.
    pub fn has_reached_goal(&self, maze: &Maze, block_size: f32) -> bool {
        let cell_x = (self.pos.x / block_size) as usize;
        let cell_y = (self.pos.y / block_size) as usize;

        // Verifica celdas vecinas
        let neighbors = [
            (cell_x.wrapping_add(1), cell_y),
            (cell_x.wrapping_sub(1), cell_y),
            (cell_x, cell_y.wrapping_add(1)),
            (cell_x, cell_y.wrapping_sub(1)),
        ];

        for (nx, ny) in neighbors {
            if let Some(ch) = maze.get(nx, ny) {
                if ch == 'g' {
                    return true;
                }
            }
        }

        false
    }

    /// Renderiza el jugador en la vista 2D como un punto con una linea de direccion.
    #[allow(dead_code)]
    pub fn render_2d(&self, fb: &mut Framebuffer, block_size: f32) {
        // escala para que el laberinto quepa
        let maze_width_pixels = 25.0 * block_size;
        let maze_height_pixels = 17.0 * block_size;
        let scale_x = fb.width as f32 / maze_width_pixels;
        let scale_y = fb.height as f32 / maze_height_pixels;
        let scale = scale_x.min(scale_y);

        // Posicion del jugador
        let screen_x = (self.pos.x * scale) as i32;
        let screen_y = (self.pos.y * scale) as i32;

        // Dibuja un circulo para el jugador
        draw_circle(fb, screen_x, screen_y, 4, 0xFFFF00); // Amarillo

        // Dibuja una linea que indica la direccion de vista
        let line_length = 15.0;
        let end_x = screen_x + (self.a.cos() * line_length) as i32;
        let end_y = screen_y + (self.a.sin() * line_length) as i32;
        draw_line(fb, screen_x, screen_y, end_x, end_y, 0xFFFF00);
    }
}

/// Dibuja un circulo en el framebuffer usando el algoritmo de punto medio.
#[allow(dead_code)]
fn draw_circle(fb: &mut Framebuffer, cx: i32, cy: i32, radius: i32, color: u32) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && px < fb.width as i32 && py >= 0 && py < fb.height as i32 {
                    fb.point(px as usize, py as usize, color);
                }
            }
        }
    }
}

/// linea usando el algoritmo de Bresenham.
#[allow(dead_code)]
fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < fb.width as i32 && y >= 0 && y < fb.height as i32 {
            fb.point(x as usize, y as usize, color);
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
