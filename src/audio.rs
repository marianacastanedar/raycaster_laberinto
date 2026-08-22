use raylib::prelude::*;

/// Administra la música de fondo y los efectos de sonido.
pub struct AudioManager<'a> {
    /// Música de fondo.
    bgm: Music<'a>,
    /// Efecto de sonido de pasos.
    footstep: Sound<'a>,
    /// Efecto de sonido de victoria.
    victory: Sound<'a>,
    /// Acumulador de tiempo para los pasos.
    footstep_timer: f32,
    /// Indica si ya se reprodujo el sonido de victoria.
    victory_played: bool,
}

impl<'a> AudioManager<'a> {
    /// Carga todos los recursos de audio.
    pub fn load(audio: &'a mut RaylibAudio) -> Self {
        // música de fondo
        let mut bgm = audio.new_music("assets/audio/bgm.ogg").unwrap_or_else(|_| {
            panic!(
                "no se pudo cargar assets/audio/bgm.ogg; \
                     revisa PREPARACION_LOCAL.md"
            )
        });
        bgm.set_looping(true);
        bgm.set_volume(0.5); // Volumen inicial

        // efectos de sonido
        let footstep = audio
            .new_sound("assets/audio/footstep.ogg")
            .unwrap_or_else(|_| {
                panic!(
                    "no se pudo cargar assets/audio/footstep.ogg; \
                     revisa PREPARACION_LOCAL.md"
                )
            });

        let victory = audio
            .new_sound("assets/audio/victory.ogg")
            .unwrap_or_else(|_| {
                panic!(
                    "no se pudo cargar assets/audio/victory.ogg; \
                     revisa PREPARACION_LOCAL.md"
                )
            });

        Self {
            bgm,
            footstep,
            victory,
            footstep_timer: 0.0,
            victory_played: false,
        }
    }

    /// Inicia música de fondo.
    pub fn start_music(&mut self) {
        self.bgm.play_stream();
    }

    pub fn update_music(&mut self) {
        self.bgm.update_stream();
    }

    /// pasos: sse reproduce si el jugador se mueve
    pub fn update_footsteps(&mut self, is_moving: bool, footstep_interval: f32, dt: f32) {
        if is_moving {
            self.footstep_timer += dt;

            if self.footstep_timer >= footstep_interval {
                self.footstep.play();
                self.footstep_timer = 0.0;
            }
        } else {
            // Si el jugador se detiene, reinicia el timer
            self.footstep_timer = 0.0;
        }
    }

    /// victoria
    pub fn play_victory(&mut self) {
        if !self.victory_played {
            self.victory.play();
            self.victory_played = true;

            // Baja el volumen de la música para que se escuche
            self.bgm.set_volume(0.2);
        }
    }

    /// Reinicia el estado de victoria para poder volver a jugar.
    pub fn reset_victory(&mut self) {
        self.victory_played = false;
        self.bgm.set_volume(0.5);
    }
}
