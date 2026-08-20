use image::RgbaImage;
use std::collections::HashMap;

/// Administra la carga y muestreo de texturas de paredes.
pub struct TextureManager {
    /// Mapa de caracter de celda a imagen RGBA.
    textures: HashMap<char, RgbaImage>,
}

impl TextureManager {
    /// Carga todas las texturas desde el directorio assets/textures/.
    pub fn load() -> Self {
        let mut textures = HashMap::new();

        // Carga cada textura con manejo de errores descriptivo
        textures.insert('1', load_texture("assets/textures/wall_1_stone.png"));
        textures.insert('2', load_texture("assets/textures/wall_2_brick.png"));
        textures.insert('3', load_texture("assets/textures/wall_3_timber.png"));
        textures.insert('g', load_texture("assets/textures/goal_door.png"));

        Self { textures }
    }

    /// Obtiene el color de un texel dada la celda y las coordenadas normalizadas.
    /// tx y ty son coordenadas normalizadas en el rango [0.0, 1.0].
    pub fn sample(&self, cell_char: char, tx: f32, ty: f32) -> u32 {
        // Si no hay textura para este caracter, retorna un color por defecto
        let texture = match self.textures.get(&cell_char) {
            Some(tex) => tex,
            None => return 0x808080, // Gris generico
        };

        let width = texture.width() as f32;
        let height = texture.height() as f32;

        // Convierte coordenadas normalizadas a pixeles de la textura
        // Clampea para evitar desbordamiento por errores de redondeo
        let x = (tx * width).clamp(0.0, width - 1.0) as u32;
        let y = (ty * height).clamp(0.0, height - 1.0) as u32;

        // Lee el pixel de la textura
        let pixel = texture.get_pixel(x, y);

        // Convierte RGBA a formato 0xRRGGBB
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;

        (r << 16) | (g << 8) | b
    }
}

/// Carga una textura desde un archivo PNG y la convierte a RGBA8.
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
