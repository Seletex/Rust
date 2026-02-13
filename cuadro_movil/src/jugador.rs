use macroquad::prelude::*;

pub struct Jugador {
    pub x: f32,
    pub y: f32,
    pub velocidad: f32,
    pub tamano: f32,
}

impl Jugador {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            velocidad: 200.0,
            tamano: 50.0,
        }
    }

    pub fn actualizar(&mut self) {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Right) { self.x += self.velocidad * dt; }
        if is_key_down(KeyCode::Left)  { self.x -= self.velocidad * dt; }
        if is_key_down(KeyCode::Down)  { self.y += self.velocidad * dt; }
        if is_key_down(KeyCode::Up)    { self.y -= self.velocidad * dt; }

        self.x = self.x.clamp(0.0, screen_width() - self.tamano);
        self.y = self.y.clamp(0.0, screen_height() - self.tamano);
    }

    pub fn dibujar(&self) {
        draw_rectangle(self.x, self.y, self.tamano, self.tamano, BLUE);
    }
}