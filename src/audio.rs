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
        // Carga la música de fondo (obligatoria)
        let mut bgm = audio.new_music("assets/audio/bgm.ogg").unwrap_or_else(|_| {
            panic!(
                "no se pudo cargar assets/audio/bgm.ogg; \
                     revisa PREPARACION_LOCAL.md"
            )
        });
        bgm.set_looping(true);
        bgm.set_volume(0.5); // Volumen inicial al 50%

        // Carga los efectos de sonido
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

    /// Inicia la música de fondo.
    pub fn start_music(&mut self) {
        self.bgm.play_stream();
    }

    /// Actualiza el stream de música (debe llamarse cada frame).
    pub fn update_music(&mut self) {
        self.bgm.update_stream();
    }

    /// Actualiza el sistema de pasos.
    /// Reproduce el efecto si el jugador se está moviendo.
    pub fn update_footsteps(&mut self, is_moving: bool, footstep_interval: f32, dt: f32) {
        if is_moving {
            self.footstep_timer += dt;

            if self.footstep_timer >= footstep_interval {
                self.footstep.play();
                self.footstep_timer = 0.0;
            }
        } else {
            // Si el jugador se detiene, reinicia el timer para que el primer
            // paso al volver a caminar suene de inmediato
            self.footstep_timer = 0.0;
        }
    }

    /// Reproduce el sonido de victoria una sola vez.
    pub fn play_victory(&mut self) {
        if !self.victory_played {
            self.victory.play();
            self.victory_played = true;

            // Baja el volumen de la música para que se escuche el jingle
            self.bgm.set_volume(0.2);
        }
    }

    /// Reinicia el estado de victoria para poder volver a jugar.
    pub fn reset_victory(&mut self) {
        self.victory_played = false;

        // Restaura el volumen de la música
        self.bgm.set_volume(0.5);
    }
}
