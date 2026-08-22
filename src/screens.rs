use image::RgbaImage;

use crate::framebuffer::Framebuffer;

/// Estados del juego.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Pantalla de bienvenida.
    Welcome,
    /// Jugando.
    Playing,
    /// Pantalla de éxito.
    Success,
}

/// imagenes de fondo de las pantallas de bienvenida y éxito
pub struct ScreenImages {
    welcome: RgbaImage,
    success: RgbaImage,
}

impl ScreenImages {
    /// Carga las imagenes desde assets/images/.
    pub fn load() -> Self {
        let welcome = load_image("assets/images/imagenBienvenida.png");
        let success = load_image("assets/images/imagenExito.png");

        Self { welcome, success }
    }
}

/// Carga una imagen
fn load_image(path: &str) -> RgbaImage {
    let bytes = std::fs::read(path).unwrap_or_else(|_| {
        panic!(
            "no se pudo leer la imagen {path}; revisa PREPARACION_LOCAL.md \
             y verifica que el archivo exista en la carpeta correcta"
        )
    });

    image::load_from_memory(&bytes)
        .unwrap_or_else(|_| panic!("no se pudo decodificar la imagen {path}"))
        .to_rgba8()
}

/// Renderiza la pantalla de bienvenida.
pub fn render_welcome(fb: &mut Framebuffer, images: &ScreenImages) {
    fb.clear();
    fb.set_background_color(0x1A1A26);
    draw_image_centered(fb, &images.welcome);
    dim_overlay(fb, 0.45);

    // Título centrado
    let title = "LABERINTO RAYCASTER :)";
    draw_text_centered(fb, title, fb.height / 2 - 80, 0xFFFFFF, 2);

    // Instrucciones
    let instructions = [
        "CONTROLES:",
        "",
        "FLECHAS ARRIBA/ABAJO - SE MUEVE",
        "FLECHAS IZQUIERDA/DERECHA O MOUSE - GIRA",
        "",
        "OBJETIVO:",
        "LLEGAR A LA PUERTA (EL PUNTO ROSA)",
        "",
        "PRESIONAR ENTER PARA EMPEZAR",
        "",
        "BUENA SUERTEEE :P",
    ];

    let mut y = fb.height / 2 - 20;
    for line in &instructions {
        draw_text_centered(fb, line, y, 0xCCCCCC, 1);
        y += 25;
    }
}

/// Renderiza la pantalla de éxito.
pub fn render_success(fb: &mut Framebuffer, images: &ScreenImages) {
    fb.clear();
    fb.set_background_color(0x1A1A26);
    draw_image_centered(fb, &images.success);
    dim_overlay(fb, 0.45);

    // Mensaje de victoria
    let title = "FELICITACIONES!";
    draw_text_centered(fb, title, fb.height / 2 - 60, 0x00FF00, 2);

    let subtitle = "EXCELENTEEE";
    draw_text_centered(fb, subtitle, fb.height / 2 - 10, 0xCCCCCC, 1);

    let instruction = "SI QUIERES JUGAR DE NUEVO PON ENTER";
    draw_text_centered(fb, instruction, fb.height / 2 + 40, 0xCCCCCC, 1);
}

fn draw_image_centered(fb: &mut Framebuffer, image: &RgbaImage) {
    let img_width = image.width() as f32;
    let img_height = image.height() as f32;

    let scale = (fb.width as f32 / img_width).min(fb.height as f32 / img_height);

    let draw_width = (img_width * scale).round() as usize;
    let draw_height = (img_height * scale).round() as usize;

    let offset_x = (fb.width.saturating_sub(draw_width)) / 2;
    let offset_y = (fb.height.saturating_sub(draw_height)) / 2;

    for dy in 0..draw_height {
        let ty = ((dy as f32 / scale) as u32).min(image.height() - 1);
        for dx in 0..draw_width {
            let tx = ((dx as f32 / scale) as u32).min(image.width() - 1);
            let pixel = image.get_pixel(tx, ty);

            if pixel[3] == 0 {
                continue;
            }

            let color =
                ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
            fb.point(offset_x + dx, offset_y + dy, color);
        }
    }
}

/// Oscurece framebuffer para ver mejor
fn dim_overlay(fb: &mut Framebuffer, alpha: f32) {
    for i in (0..fb.buffer.len()).step_by(4) {
        fb.buffer[i] = (fb.buffer[i] as f32 * (1.0 - alpha)) as u8;
        fb.buffer[i + 1] = (fb.buffer[i + 1] as f32 * (1.0 - alpha)) as u8;
        fb.buffer[i + 2] = (fb.buffer[i + 2] as f32 * (1.0 - alpha)) as u8;
    }
}
fn draw_text_centered(fb: &mut Framebuffer, text: &str, y: usize, color: u32, scale: usize) {
    let char_width = 8 * scale;
    let text_width = text.len() * char_width;
    let x = (fb.width - text_width) / 2;

    draw_text(fb, text, x, y, color, scale);
}

/// Dibuja texto con bitmap
fn draw_text(fb: &mut Framebuffer, text: &str, x: usize, y: usize, color: u32, scale: usize) {
    let mut current_x = x;

    for ch in text.chars() {
        draw_char(fb, ch, current_x, y, color, scale);
        current_x += 8 * scale;
    }
}

/// Dibuja un carácter con bitmap de 8x8.
fn draw_char(fb: &mut Framebuffer, ch: char, x: usize, y: usize, color: u32, scale: usize) {
    let glyph = get_glyph(ch);

    for (row, &byte) in glyph.iter().enumerate() {
        for col in 0..8 {
            if (byte >> (7 - col)) & 1 == 1 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + col * scale + sx;
                        let py = y + row * scale + sy;
                        if px < fb.width && py < fb.height {
                            fb.point(px, py, color);
                        }
                    }
                }
            }
        }
    }
}


fn get_glyph(ch: char) -> [u8; 8] {
    match ch {
        'A' => [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
        'B' => [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
        'C' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
        'D' => [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
        'E' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
        'F' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
        'G' => [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
        'H' => [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
        'I' => [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        'J' => [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
        'K' => [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
        'L' => [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
        'M' => [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
        'N' => [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
        'O' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        'P' => [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
        'Q' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x0E, 0x00],
        'R' => [0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00],
        'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        'W' => [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
        'X' => [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
        'Y' => [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
        'Z' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
        '0' => [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
        '1' => [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
        '2' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x7E, 0x00],
        '3' => [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
        '4' => [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],
        '5' => [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
        '6' => [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
        '7' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
        '8' => [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
        '9' => [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
        ':' => [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00],
        '/' => [0x00, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x00, 0x00],
        '-' => [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
        '!' => [0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
