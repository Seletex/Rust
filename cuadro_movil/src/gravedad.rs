#[derive(Debug, Clone, Copy)]
pub struct Gravedad {
    pub velocidad_y: f32,
    pub aceleracion: f32,
    /// Velocidad terminal (magnitud). Use `f32::INFINITY` for sin límite.
    pub velocidad_terminal: f32,
}

impl Gravedad {
    pub fn new(aceleracion: f32) -> Self {
        Self {
            velocidad_y: 0.0,
            aceleracion,
            velocidad_terminal: f32::INFINITY,
        }
    }

    #[inline]
    pub fn aplicar(&mut self, y: &mut f32, dt: f32) {
        self.velocidad_y += self.aceleracion * dt;
        if self.velocidad_terminal.is_finite() {
            self.velocidad_y = self.velocidad_y.clamp(-self.velocidad_terminal, self.velocidad_terminal);
        }
        *y += self.velocidad_y * dt;
    }

    #[inline]
    pub fn saltar(&mut self, fuerza: f32) {
        let fuerza_pos = fuerza.abs();
        let v = if self.velocidad_terminal.is_finite() {
            fuerza_pos.min(self.velocidad_terminal)
        } else {
            fuerza_pos
        };
        self.velocidad_y = -v;
    }

    #[inline]
    pub fn detener(&mut self) {
        self.velocidad_y = 0.0;
    }
}

impl Default for Gravedad {
    fn default() -> Self {
        // Valor por defecto razonable para gravedad en juegos (m/s^2)
        Self::new(9.81)
    }
}