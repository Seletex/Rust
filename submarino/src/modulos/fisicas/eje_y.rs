// FISICAS EJE Y (GRAVEDAD, EMPUJE Y FRICCION)
pub struct FisicasEjeY {
    pub posicion: f32,
    pub velocidad: f32,
    pub masa: f32,
    pub gravedad: f32,
    pub volumen: f32,
    pub densidad_agua: f32,
    pub friccion: f32,
    pub dt: f32, // delta time
}

impl FisicasEjeY {
    pub fn new(posicion_inicial: f32) -> Self {
        Self {
            posicion: posicion_inicial,
            velocidad: 0.0,
            masa: 100.0,
            gravedad: 9.8,
            volumen: 0.001,
            densidad_agua: 10.0,
            friccion: 0.01,
            dt: 0.1,
        }
    }

    pub fn actualizar(&mut self) {
        let f_gravedad = self.masa * self.gravedad;
        let f_flotabilidad = self.densidad_agua * self.volumen * self.gravedad;
        
        let f_friccion = if self.velocidad.abs() > 0.0001 {
            -(self.velocidad.signum()) * self.friccion * self.velocidad
        } else {
            0.0
        };

        let fuerza_neta = f_gravedad - f_flotabilidad + f_friccion;

        println!("Gravedad: {:.2}N | Flotabilidad: {:.2}N | Fricción: {:.2}N", f_gravedad, f_flotabilidad, f_friccion);
        println!("Fuerza Neta: {:.2}N", fuerza_neta);

        let aceleracion = fuerza_neta / self.masa;

        let nueva_velocidad = self.velocidad + aceleracion * self.dt;

        if self.velocidad.abs() > 0.0 && nueva_velocidad.signum() != self.velocidad.signum() {
            if f_friccion.abs() > (f_gravedad - f_flotabilidad).abs() {
                self.velocidad = 0.0;
            } else {
                self.velocidad = nueva_velocidad;
            }
        } else {
            self.velocidad = nueva_velocidad;
        }

        self.posicion += self.velocidad * self.dt;
        println!("Velocidad: {:.4} | Posición Y: {:.4}", self.velocidad, self.posicion);
    }
}

