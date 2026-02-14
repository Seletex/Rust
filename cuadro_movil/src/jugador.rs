use macroquad::prelude::*;
use crate::gravedad::Gravedad;

pub struct Jugador {
    pub x: f32,
    pub y: f32,
    pub velocidad: f32,
    pub tamano: f32,
    pub gravedad: Gravedad,
}

impl Jugador {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            velocidad: 200.0,
            tamano: 50.0,
            gravedad: Gravedad::new(500.0), // Ajusta la aceleración a gusto
        }
    }

    pub fn actualizar(&mut self) {
        let dt = get_frame_time();

        // Movimiento Horizontal
        if is_key_down(KeyCode::Right) { self.x += self.velocidad * dt; }
        if is_key_down(KeyCode::Left)  { self.x -= self.velocidad * dt; }
        
        // Aplicar Gravedad
        self.gravedad.aplicar(&mut self.y, dt);

        // Colisión con el suelo (simple)
        if self.y > screen_height() - self.tamano {
            self.y = screen_height() - self.tamano;
            self.gravedad.detener();

            // Saltar solo si está en el suelo
            if is_key_pressed(KeyCode::Up) { // Cambiado a KeyPressed para evitar saltos múltiples
                self.gravedad.saltar(400.0); // Fuerza de salto
            }
        }
        
        // Limites de pantalla (Horizontal y Techo)
        self.x = self.x.clamp(0.0, screen_width() - self.tamano);
        if self.y < 0.0 { self.y = 0.0; self.gravedad.velocidad_y = 0.0; } // Chocar con el techo detiene la subida
    }

    pub fn dibujar(&self) {
        draw_rectangle(self.x, self.y, self.tamano, self.tamano, BLUE);
    }
}