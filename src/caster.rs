use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

/// Resultado del lanzamiento de un rayo.
struct RayHit {
    /// Distancia desde el jugador hasta el punto de impacto (sin corregir).
    distance: f32,
    /// Caracter de la celda que se golpeo.
    cell_char: char,
    /// True si se golpeo una cara vertical, false si es horizontal.
    is_vertical: bool,
}

/// Renderiza la escena en 3D usando raycasting.
pub fn render_3d(
    fb: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    fov: f32,
    block_size: f32,
    ceiling_color: u32,
    floor_color: u32,
) -> Vec<f32> {
    let screen_width = fb.width;
    let screen_height = fb.height;

    // Calcula la distancia al plano de proyección
    let projection_distance = (screen_width as f32 / 2.0) / (fov / 2.0).tan();

    // Z-buffer para los sprites (distancia corregida por columna)
    let mut zbuffer = vec![0.0; screen_width];

    // Lanza un rayo por cada columna de pantalla
    for (x, zbuffer_slot) in zbuffer.iter_mut().enumerate() {
        // Calcula el angulo del rayo
        let ray_angle = player.a - fov / 2.0 + fov * (x as f32 / screen_width as f32);

        // Lanza el rayo y obtiene el impacto
        let hit = cast_ray(player, maze, ray_angle, block_size);

        // Corrige el efecto de ojo de pez
        let corrected_distance = hit.distance * (ray_angle - player.a).cos();
        *zbuffer_slot = corrected_distance;

        // Calcula la altura de la pared en pantalla
        let wall_height = (block_size / corrected_distance) * projection_distance;
        let wall_top = (screen_height as f32 / 2.0 - wall_height / 2.0) as i32;
        let wall_bottom = (screen_height as f32 / 2.0 + wall_height / 2.0) as i32;

        // Obtiene el color base de la pared
        let base_color = get_wall_color(hit.cell_char);

        // Aplica sombreado por distancia y orientacion
        let wall_color = apply_shading(base_color, corrected_distance, hit.is_vertical);

        // Dibuja la columna: techo, pared, piso
        for y in 0..screen_height {
            let y_i32 = y as i32;
            let color = if y_i32 < wall_top {
                ceiling_color
            } else if y_i32 >= wall_top && y_i32 < wall_bottom {
                wall_color
            } else {
                floor_color
            };
            fb.point(x, y, color);
        }
    }

    zbuffer
}

/// Lanza un rayo desde la posicion del jugador en el angulo especificado
/// y devuelve informacion sobre el primer impacto con una pared.
fn cast_ray(player: &Player, maze: &Maze, angle: f32, block_size: f32) -> RayHit {
    let step_size = 0.5; // Paso pequeño para precision
    let max_distance = 1000.0; // Distancia maxima para evitar loops infinitos

    let dir_x = angle.cos();
    let dir_y = angle.sin();

    let mut distance = 0.0;
    let mut prev_cell_x = (player.pos.x / block_size) as usize;

    loop {
        distance += step_size;

        if distance > max_distance {
            // No se encontro pared, retorna un impacto lejano
            return RayHit {
                distance: max_distance,
                cell_char: '1',
                is_vertical: true,
            };
        }

        let ray_x = player.pos.x + dir_x * distance;
        let ray_y = player.pos.y + dir_y * distance;

        let cell_x = (ray_x / block_size) as usize;
        let cell_y = (ray_y / block_size) as usize;

        // Verifica si la celda actual es solida
        if maze.is_solid(cell_x, cell_y) {
            // Determina que cara se golpeo comparando con la celda anterior
            let is_vertical = cell_x != prev_cell_x;

            // Obtiene el caracter de la celda
            let cell_char = maze.get(cell_x, cell_y).unwrap_or('1');

            return RayHit {
                distance,
                cell_char,
                is_vertical,
            };
        }

        prev_cell_x = cell_x;
    }
}

/// Devuelve el color base de una pared segun su tipo.
fn get_wall_color(ch: char) -> u32 {
    match ch {
        '1' => 0x8B8B8B, // Gris claro (piedra)
        '2' => 0xD2691E, // Naranja chocolate (ladrillo)
        '3' => 0xA0522D, // Sienna (madera)
        'g' => 0x00FF00, // Verde brillante (meta)
        _ => 0x808080,   // Gris generico
    }
}

/// Aplica sombreado por distancia y orientacion de la cara.
/// Las caras horizontales se oscurecen mas que las verticales para dar profundidad.
fn apply_shading(color: u32, distance: f32, is_vertical: bool) -> u32 {
    // Factor de oscurecimiento por distancia (0.0 = muy lejos, 1.0 = muy cerca)
    let distance_factor = (1.0 - (distance / 500.0).min(0.8)).max(0.2);

    // Las caras horizontales se oscurecen un 20% adicional
    let orientation_factor = if is_vertical { 1.0 } else { 0.8 };

    let final_factor = distance_factor * orientation_factor;

    // Descompone el color en componentes RGB
    let r = ((color >> 16) & 0xFF) as f32;
    let g = ((color >> 8) & 0xFF) as f32;
    let b = (color & 0xFF) as f32;

    // Aplica el sombreado
    let r_shaded = (r * final_factor) as u32;
    let g_shaded = (g * final_factor) as u32;
    let b_shaded = (b * final_factor) as u32;

    (r_shaded << 16) | (g_shaded << 8) | b_shaded
}
