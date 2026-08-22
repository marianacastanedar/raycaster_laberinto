use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

/// Renderiza un minimapa en la esquina superior derecha del framebuffer.
pub fn render_minimap(fb: &mut Framebuffer, maze: &Maze, player: &Player, block_size: f32) {
    let minimap_size = 150; // Tamano del minimapa
    let margin = 10;
    let minimap_x = fb.width - minimap_size - margin;
    let minimap_y = margin;

    // escala para que el laberinto quepa en el minimapa
    let maze_width_pixels = maze.width as f32 * block_size;
    let maze_height_pixels = maze.height as f32 * block_size;
    let scale_x = minimap_size as f32 / maze_width_pixels;
    let scale_y = minimap_size as f32 / maze_height_pixels;
    let scale = scale_x.min(scale_y);

    // fondo para el minimapa
    for y in 0..minimap_size {
        for x in 0..minimap_size {
            let px = minimap_x + x;
            let py = minimap_y + y;
            if px < fb.width && py < fb.height {
                fb.point(px, py, 0x0A0A0A); // Casi negro
            }
        }
    }

    // Dibuja las celdas del laberinto
    for maze_y in 0..maze.height {
        for maze_x in 0..maze.width {
            let ch = maze.data[maze_y][maze_x];
            let color = get_minimap_color(ch);

            let cell_screen_x = (maze_x as f32 * block_size * scale) as usize;
            let cell_screen_y = (maze_y as f32 * block_size * scale) as usize;
            let cell_width = (block_size * scale).max(1.0) as usize;
            let cell_height = (block_size * scale).max(1.0) as usize;

            // Dibuja la celda en el minimapa
            for dy in 0..cell_height {
                for dx in 0..cell_width {
                    let px = minimap_x + cell_screen_x + dx;
                    let py = minimap_y + cell_screen_y + dy;
                    if px < fb.width && py < fb.height {
                        fb.point(px, py, color);
                    }
                }
            }
        }
    }

    // Dibuja el jugador como un punto con linea de direccion
    let player_screen_x = minimap_x + (player.pos.x * scale) as usize;
    let player_screen_y = minimap_y + (player.pos.y * scale) as usize;

    // Punto del jugador
    draw_circle_minimap(
        fb,
        player_screen_x as i32,
        player_screen_y as i32,
        3,
        0xeeafff,
    );

    // Linea de direccion
    let dir_length = 8.0;
    let end_x = player_screen_x as i32 + (player.a.cos() * dir_length) as i32;
    let end_y = player_screen_y as i32 + (player.a.sin() * dir_length) as i32;
    draw_line_minimap(
        fb,
        player_screen_x as i32,
        player_screen_y as i32,
        end_x,
        end_y,
        0xFFFF00,
    );

    // borde blanco alrededor del minimapa
    draw_rect_outline(
        fb,
        minimap_x,
        minimap_y,
        minimap_size,
        minimap_size,
        0xFFFFFF,
    );
}

/// color de una celda en el minimapa.
fn get_minimap_color(ch: char) -> u32 {
    match ch {
        '1' | '2' | '3' => 0x404040, // Gris oscuro para todas las paredes
        'g' => 0xff43b3,             //la meta
        _ => 0x1A1A1A,               // Muy oscuro para piso
    }
}

/// Dibuja un circulo en el minimapa.
fn draw_circle_minimap(fb: &mut Framebuffer, cx: i32, cy: i32, radius: i32, color: u32) {
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

/// Dibuja una linea en el minimapa usando Bresenham.
fn draw_line_minimap(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
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

/// Dibuja el contorno de un rectangulo.
fn draw_rect_outline(fb: &mut Framebuffer, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for dx in 0..w {
        if x + dx < fb.width && y < fb.height {
            fb.point(x + dx, y, color);
        }
    }
    for dx in 0..w {
        if x + dx < fb.width && y + h - 1 < fb.height {
            fb.point(x + dx, y + h - 1, color);
        }
    }
    for dy in 0..h {
        if x < fb.width && y + dy < fb.height {
            fb.point(x, y + dy, color);
        }
    }
    for dy in 0..h {
        if x + w - 1 < fb.width && y + dy < fb.height {
            fb.point(x + w - 1, y + dy, color);
        }
    }
}
