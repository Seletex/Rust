#[derive(Debug, Clone, Copy)]
pub struct Motor2D {
    pub velocidad_x: f32,
    pub velocidad_y: f32,
    pub aceleracion_y: f32,
    pub velocidad_terminal: f32,
    /// Fricción o resistencia del medio (1.0 = sin friccion, 0.9 = mucha friccion)
    pub friccion: f32,
}

impl Motor2D {
    pub fn new(aceleracion_y: f32) -> Self {
        Self {
            velocidad_x: 0.0,
            velocidad_y: 0.0,
            aceleracion_y,
            velocidad_terminal: f32::INFINITY,
            friccion: 1.0,
        }
    }

    #[inline]
    pub fn aplicar(&mut self, x: &mut f32, y: &mut f32, dt: f32) {
        // Aplicar gravedad/flotabilidad solo en Y
        self.velocidad_y += self.aceleracion_y * dt;
        
        // Aplicar fricción en ambos ejes (resistencia al agua)
        if self.friccion < 1.0 {
            let factor_friccion = self.friccion.powf(dt * 60.0);
            self.velocidad_x *= factor_friccion;
            self.velocidad_y *= factor_friccion;
        }

        // Limitar velocidades
        if self.velocidad_terminal.is_finite() {
            self.velocidad_x = self.velocidad_x.clamp(-self.velocidad_terminal, self.velocidad_terminal);
            self.velocidad_y = self.velocidad_y.clamp(-self.velocidad_terminal, self.velocidad_terminal);
        }

        // Actualizar posiciones
        *x += self.velocidad_x * dt;
        *y += self.velocidad_y * dt;
    }

    #[inline]
    pub fn impulsar_x(&mut self, fuerza: f32) {
        self.velocidad_x += fuerza;
    }

    #[inline]
    pub fn impulsar_y(&mut self, fuerza: f32) {
        self.velocidad_y += fuerza;
    }

    #[inline]
    pub fn detener(&mut self) {
        self.velocidad_x = 0.0;
        self.velocidad_y = 0.0;
    }
}

impl Default for Motor2D {
    fn default() -> Self {
        Self::new(9.81)
    }
}