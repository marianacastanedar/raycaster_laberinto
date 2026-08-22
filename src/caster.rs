use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::textures::TextureManager;

struct RayHit {
    /// Distancia: el jugador hasta el punto de impacto
    distance: f32,
    /// celda que se golpea
    cell_char: char,
    /// True si se golpeo una cara vertical, false si es horizontal.
    is_vertical: bool,
    /// Posicion X del impacto
    hit_x: f32,
    /// Posicion Y del impacto
    hit_y: f32,
}

/// Renderiza en 3D usando raycasting
#[allow(clippy::too_many_arguments)]
pub fn render_3d(
    fb: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    textures: &TextureManager,
    fov: f32,
    block_size: f32,
    ceiling_color: u32,
    floor_color: u32,
) -> Vec<f32> {
    let screen_width = fb.width;
    let screen_height = fb.height;
    let projection_distance = (screen_width as f32 / 2.0) / (fov / 2.0).tan();

    // Z-buffer para los sprites
    let mut zbuffer = vec![0.0; screen_width];

    // Lanza un rayo por cada columna de pantalla
    for (x, zbuffer_slot) in zbuffer.iter_mut().enumerate() {
        // Calcula el angulo
        let ray_angle = player.a - fov / 2.0 + fov * (x as f32 / screen_width as f32);
        let hit = cast_ray(player, maze, ray_angle, block_size);

        // Corrige el efecto de ojo de pez
        let corrected_distance = hit.distance * (ray_angle - player.a).cos();
        *zbuffer_slot = corrected_distance;

        // Calcula la altura de la pared en pantalla
        let wall_height = (block_size / corrected_distance) * projection_distance;
        let wall_top = (screen_height as f32 / 2.0 - wall_height / 2.0) as i32;
        let wall_bottom = (screen_height as f32 / 2.0 + wall_height / 2.0) as i32;

        let tx = if hit.is_vertical {
            (hit.hit_y % block_size) / block_size
        } else {
            (hit.hit_x % block_size) / block_size
        };

        // Dibuja la columna: techo, pared, piso
        for y in 0..screen_height {
            let y_i32 = y as i32;
            let color = if y_i32 < wall_top {
                ceiling_color
            } else if y_i32 >= wall_top && y_i32 < wall_bottom {
                let ty = (y_i32 - wall_top) as f32 / wall_height;
                let texel_color = textures.sample(hit.cell_char, tx, ty);

                // Aplica sombreado por distancia y orientacion
                apply_shading(texel_color, corrected_distance, hit.is_vertical)
            } else {
                floor_color
            };
            fb.point(x, y, color);
        }
    }

    zbuffer
}

/// Lanza un rayo desde la posicion del jugador en el angulo y devuelve informacion sobre el impacto con pared.
fn cast_ray(player: &Player, maze: &Maze, angle: f32, block_size: f32) -> RayHit {
    let step_size = 0.5; // Paso pequeño
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
                hit_x: player.pos.x,
                hit_y: player.pos.y,
            };
        }

        let ray_x = player.pos.x + dir_x * distance;
        let ray_y = player.pos.y + dir_y * distance;

        let cell_x = (ray_x / block_size) as usize;
        let cell_y = (ray_y / block_size) as usize;

        // celda actual es solida?
        if maze.is_solid(cell_x, cell_y) {
            // Determina que cara se golpeo comparando con la celda anterior
            let is_vertical = cell_x != prev_cell_x;
            let cell_char = maze.get(cell_x, cell_y).unwrap_or('1');

            return RayHit {
                distance,
                cell_char,
                is_vertical,
                hit_x: ray_x,
                hit_y: ray_y,
            };
        }

        prev_cell_x = cell_x;
    }
}

/// Aplica sombreado por distancia y orientacion
fn apply_shading(color: u32, distance: f32, is_vertical: bool) -> u32 {
    let distance_factor = (1.0 - (distance / 500.0).min(0.8)).max(0.2);
    let orientation_factor = if is_vertical { 1.0 } else { 0.8 };

    let final_factor = distance_factor * orientation_factor;

    let r = ((color >> 16) & 0xFF) as f32;
    let g = ((color >> 8) & 0xFF) as f32;
    let b = (color & 0xFF) as f32;

    // Aplica el sombreado
    let r_shaded = (r * final_factor) as u32;
    let g_shaded = (g * final_factor) as u32;
    let b_shaded = (b * final_factor) as u32;

    (r_shaded << 16) | (g_shaded << 8) | b_shaded
}
