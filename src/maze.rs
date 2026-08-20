use std::fs;

use nalgebra_glm::Vec2;

use crate::framebuffer::Framebuffer;

/// Representa el laberinto cargado desde maze.txt.
pub struct Maze {
    /// Matriz de caracteres del laberinto. Cada caracter representa un tipo de celda.
    pub data: Vec<Vec<char>>,
    /// Ancho del laberinto en celdas.
    pub width: usize,
    /// Alto del laberinto en celdas.
    pub height: usize,
    /// Posicion inicial del jugador en coordenadas de mundo (pixeles).
    pub player_start: Vec2,
    /// Posiciones de las llamas en coordenadas de mundo (pixeles).
    #[allow(dead_code)]
    pub fire_positions: Vec<Vec2>,
}

impl Maze {
    /// Carga el laberinto desde un archivo de texto.
    /// Los caracteres 'p' (jugador) y 'f' (llama) se extraen y reemplazan con ' '.
    pub fn load(path: &str, block_size: f32) -> Self {
        let content = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "no se pudo leer el archivo {path}; revisa que exista y este en la raiz del proyecto"
            )
        });

        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = if height > 0 { lines[0].len() } else { 0 };

        let mut data = vec![vec![' '; width]; height];
        let mut player_start = Vec2::new(0.0, 0.0);
        let mut fire_positions: Vec<Vec2> = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if x >= width {
                    break;
                }

                match ch {
                    'p' => {
                        // Guarda la posicion del jugador en coordenadas de mundo
                        // (centro de la celda)
                        player_start = Vec2::new(
                            x as f32 * block_size + block_size / 2.0,
                            y as f32 * block_size + block_size / 2.0,
                        );
                        data[y][x] = ' ';
                    }
                    'f' => {
                        // Guarda la posicion de la llama en coordenadas de mundo
                        fire_positions.push(Vec2::new(
                            x as f32 * block_size + block_size / 2.0,
                            y as f32 * block_size + block_size / 2.0,
                        ));
                        data[y][x] = ' ';
                    }
                    _ => {
                        data[y][x] = ch;
                    }
                }
            }
        }

        Self {
            data,
            width,
            height,
            player_start,
            fire_positions,
        }
    }

    /// Devuelve el caracter en la celda especificada.
    /// Retorna None si la posicion esta fuera de rango.
    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if y < self.height && x < self.width {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    /// Verifica si una celda es solida (no se puede atravesar).
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            Some(' ') => false,
            Some(_) => true,
            None => true, // Fuera de rango se considera solido
        }
    }

    /// Renderiza el laberinto completo en vista 2D cenital.
    /// Escala el laberinto para que quepa en el framebuffer.
    pub fn render_2d(&self, fb: &mut Framebuffer, block_size: f32) {
        // Calcula el factor de escala para que el laberinto quepa en pantalla
        let scale_x = fb.width as f32 / (self.width as f32 * block_size);
        let scale_y = fb.height as f32 / (self.height as f32 * block_size);
        let scale = scale_x.min(scale_y);

        for y in 0..self.height {
            for x in 0..self.width {
                let ch = self.data[y][x];
                let color = get_cell_color(ch);

                // Calcula las coordenadas en pantalla
                let screen_x = (x as f32 * block_size * scale) as usize;
                let screen_y = (y as f32 * block_size * scale) as usize;
                let cell_width = (block_size * scale) as usize;
                let cell_height = (block_size * scale) as usize;

                // Dibuja un rectangulo solido para esta celda
                draw_rect(fb, screen_x, screen_y, cell_width, cell_height, color);
            }
        }
    }
}

/// Devuelve el color asociado a un tipo de celda.
fn get_cell_color(ch: char) -> u32 {
    match ch {
        '1' => 0x5A5A5A, // Gris oscuro (piedra)
        '2' => 0xC87533, // Naranja terracota (ladrillo)
        '3' => 0x8B4513, // Marron (madera)
        'g' => 0x00FF00, // Verde brillante (meta)
        _ => 0x1A1A1A,   // Casi negro (piso)
    }
}

/// Dibuja un rectangulo solido en el framebuffer.
fn draw_rect(fb: &mut Framebuffer, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for dy in 0..h {
        for dx in 0..w {
            fb.point(x + dx, y + dy, color);
        }
    }
}
