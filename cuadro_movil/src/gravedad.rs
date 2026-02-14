pub struct Gravedad {
    pub velocidad_y: f32,
    pub aceleracion: f32,
}

impl Gravedad {
    pub fn new(aceleracion: f32) -> Self {
        Self {
            velocidad_y: 0.0,
            aceleracion,
        }
    }

    pub fn aplicar(&mut self, y: &mut f32, dt: f32) {
        self.velocidad_y += self.aceleracion * dt;
        *y += self.velocidad_y * dt;
    }

    pub fn saltar(&mut self, fuerza: f32) {
        self.velocidad_y = -fuerza;
    }

    pub fn detener(&mut self) {
        self.velocidad_y = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravedad_aplicar() {
        let mut gravedad = Gravedad::new(10.0);
        let mut y = 0.0;
        let dt = 1.0;

        // t=0: v=0, y=0
        // t=1: v=10, y=10 (v*dt) -> 10*1 = 10
        gravedad.aplicar(&mut y, dt);
        assert_eq!(gravedad.velocidad_y, 10.0);
        assert_eq!(y, 10.0);

        // t=2: v=20, y=10 + 20*1 = 30
        gravedad.aplicar(&mut y, dt);
        assert_eq!(gravedad.velocidad_y, 20.0);
        assert_eq!(y, 30.0);
    }

    #[test]
    fn test_salto() {
        let mut gravedad = Gravedad::new(10.0);
        gravedad.saltar(50.0);
        assert_eq!(gravedad.velocidad_y, -50.0);
    }
}
