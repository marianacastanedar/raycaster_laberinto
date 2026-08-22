use image::RgbaImage;
use std::collections::HashMap;

pub struct TextureManager {
    textures: HashMap<char, RgbaImage>,
}

impl TextureManager {
    /// Carga todas las texturas
    pub fn load() -> Self {
        let mut textures = HashMap::new();

        textures.insert('1', load_texture("assets/textures/wall_1_stone.png"));
        textures.insert('2', load_texture("assets/textures/wall_2_brick.png"));
        textures.insert('3', load_texture("assets/textures/wall_3_timber.png"));
        textures.insert('g', load_texture("assets/textures/goal_door.png"));

        Self { textures }
    }

    pub fn sample(&self, cell_char: char, tx: f32, ty: f32) -> u32 {
        // Si no hay textura es un color por defecto
        let texture = match self.textures.get(&cell_char) {
            Some(tex) => tex,
            None => return 0x808080, // Gris generico
        };

        let width = texture.width() as f32;
        let height = texture.height() as f32;

        // Coordenadas a pixeles de la textura
        let x = (tx * width).clamp(0.0, width - 1.0) as u32;
        let y = (ty * height).clamp(0.0, height - 1.0) as u32;

        // Lee el pixel
        let pixel = texture.get_pixel(x, y);
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;

        (r << 16) | (g << 8) | b
    }
}

/// Carga textura desde png
fn load_texture(path: &str) -> RgbaImage {
    image::open(path)
        .unwrap_or_else(|_| {
            panic!(
                "no se pudo cargar la textura {path}; revisa PREPARACION_LOCAL.md \
                 y verifica que el archivo exista en la carpeta correcta"
            )
        })
        .to_rgba8()
}
