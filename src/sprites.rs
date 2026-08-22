use image::RgbaImage;
use nalgebra_glm::Vec2;

use crate::framebuffer::Framebuffer;
use crate::player::Player;

/// Administra la textura de fuego y el renderizado de sprites.
pub struct SpriteManager {
    fire_texture: RgbaImage,
}

impl SpriteManager {
    /// Carga la textura desde assets/sprites/.
    pub fn load() -> Self {
        let fire_texture = image::open("assets/sprites/fire.png")
            .unwrap_or_else(|_| {
                panic!(
                    "no se pudo cargar assets/sprites/fire.png; \
                     revisa PREPARACION_LOCAL.md y verifica que el archivo exista"
                )
            })
            .to_rgba8();

        Self { fire_texture }
    }

    /// Renderiza todos los sprites de fuego en el framebuffer.
    /// Usa el z-buffer para ocultar sprites detras de paredes.
    #[allow(clippy::too_many_arguments)]
    pub fn render_sprites(
        &self,
        fb: &mut Framebuffer,
        player: &Player,
        fire_positions: &[Vec2],
        zbuffer: &[f32],
        fov: f32,
        block_size: f32,
        fire_scale: f32,
    ) {
        let screen_width = fb.width;
        let screen_height = fb.height;

        // Calcula la distancia
        let projection_distance = (screen_width as f32 / 2.0) / (fov / 2.0).tan();

        // Prepara una lista de sprites con su distancia para ordenarlos
        let mut sprites_with_distance: Vec<(usize, f32)> = fire_positions
            .iter()
            .enumerate()
            .map(|(i, pos)| {
                let dx = pos.x - player.pos.x;
                let dy = pos.y - player.pos.y;
                let distance = (dx * dx + dy * dy).sqrt();
                (i, distance)
            })
            .collect();

        // Ordena de mas lejano a mas cercano
        sprites_with_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Renderiza cada sprite
        for (i, distance) in sprites_with_distance {
            let fire_pos = fire_positions[i];

            // Calcula el angulo
            let dx = fire_pos.x - player.pos.x;
            let dy = fire_pos.y - player.pos.y;
            let mut angle = dy.atan2(dx) - player.a;

            while angle > std::f32::consts::PI {
                angle -= std::f32::consts::TAU;
            }
            while angle < -std::f32::consts::PI {
                angle += std::f32::consts::TAU;
            }

            let fov_margin = fov / 2.0 + 0.2; 
            if angle.abs() > fov_margin {
                continue;
            }

            let sprite_height = (block_size / distance) * projection_distance * fire_scale;
            let sprite_width = sprite_height
                * (self.fire_texture.width() as f32 / self.fire_texture.height() as f32);
            let center_x = (screen_width as f32 / 2.0) + angle.tan() * projection_distance;
            let floor_line = (screen_height as f32 / 2.0)
                + ((block_size / distance) * projection_distance) / 2.0;
            let sprite_bottom = floor_line;
            let sprite_top = sprite_bottom - sprite_height;
            let start_x = (center_x - sprite_width / 2.0) as i32;
            let end_x = (center_x + sprite_width / 2.0) as i32;
            let start_y = sprite_top as i32;
            let end_y = sprite_bottom as i32;

            // Dibuja cada columna del sprite
            for screen_x in start_x.max(0)..end_x.min(screen_width as i32) {
                if screen_x >= 0
                    && (screen_x as usize) < zbuffer.len()
                    && distance >= zbuffer[screen_x as usize]
                {
                    continue;
                }

                let tx = (screen_x - start_x) as f32 / sprite_width;
                let texture_x = (tx * self.fire_texture.width() as f32)
                    .clamp(0.0, self.fire_texture.width() as f32 - 1.0)
                    as u32;

                // Dibuja cada pixel de la columna
                for screen_y in start_y.max(0)..end_y.min(screen_height as i32) {
                    let ty = (screen_y - start_y) as f32 / sprite_height;
                    let texture_y = (ty * self.fire_texture.height() as f32)
                        .clamp(0.0, self.fire_texture.height() as f32 - 1.0)
                        as u32;

                    let pixel = self.fire_texture.get_pixel(texture_x, texture_y);
                    if pixel[3] == 0 {
                        continue;
                    }
                    let color =
                        ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                    // Dibuja el pixel
                    if screen_x >= 0
                        && screen_x < screen_width as i32
                        && screen_y >= 0
                        && screen_y < screen_height as i32
                    {
                        fb.point(screen_x as usize, screen_y as usize, color);
                    }
                }
            }
        }
    }
}
