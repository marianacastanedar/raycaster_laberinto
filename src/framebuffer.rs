/// Buffer de pixeles en memoria que se renderiza a una textura de raylib.
/// Almacena los pixeles en formato RGBA de 4 bytes por pixel.
pub struct Framebuffer {
    #[allow(dead_code)]
    pub width: usize,
    #[allow(dead_code)]
    pub height: usize,
    /// 4 bytes por pixel, en orden R, G, B, A.
    pub buffer: Vec<u8>,
    background_color: u32,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con las dimensiones especificadas.
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height * 4];
        Self {
            width,
            height,
            buffer,
            background_color: 0x000000,
        }
    }

    /// Establece el color de fondo que se usa al limpiar el buffer.
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    /// Limpia todo el buffer con el color de fondo.
    pub fn clear(&mut self) {
        let r = ((self.background_color >> 16) & 0xFF) as u8;
        let g = ((self.background_color >> 8) & 0xFF) as u8;
        let b = (self.background_color & 0xFF) as u8;

        for i in (0..self.buffer.len()).step_by(4) {
            self.buffer[i] = r;
            self.buffer[i + 1] = g;
            self.buffer[i + 2] = b;
            self.buffer[i + 3] = 255;
        }
    }

    /// Escribe un pixel en la posicion especificada.
    /// El color entra como 0xRRGGBB para que las constantes se lean como hexadecimales normales.
    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) * 4;
        self.buffer[idx] = ((color >> 16) & 0xFF) as u8;
        self.buffer[idx + 1] = ((color >> 8) & 0xFF) as u8;
        self.buffer[idx + 2] = (color & 0xFF) as u8;
        self.buffer[idx + 3] = 255;
    }
}
