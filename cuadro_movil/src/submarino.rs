use macroquad::prelude::*;
use crate::gravedad::Motor2D;

pub struct Particula {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub vida: f32,
}

/// Representa el vehículo físico y sus efectos visuales.
pub struct Submarino {
    pub motor: Motor2D,
    pub tamano: f32,
    estela: Vec<Particula>,
    angulo_helice: f32,
    volteo: f32,
}

impl Submarino {
    pub fn new(tamano: f32) -> Self {
        let mut motor = Motor2D::new(30.0);
        motor.velocidad_terminal = 600.0;
        motor.friccion = 0.985;

        Self {
            motor,
            tamano,
            estela: Vec::new(),
            angulo_helice: 0.0,
            volteo: 1.0,
        }
    }

    pub fn aplicar_fisica_agua(&mut self, x: &mut f32, y: &mut f32, dt: f32) {
        let factor_profundidad = *y / screen_height();
        self.motor.friccion = 0.985 - (factor_profundidad * 0.015);
        self.motor.aplicar(x, y, dt);
    }

    pub fn propulsar_x(&mut self, fuerza: f32) {
        self.motor.impulsar_x(fuerza);
    }

    pub fn propulsar_y(&mut self, fuerza: f32, y: f32) {
        let factor_profundidad = y / screen_height();
        let resistencia = if fuerza > 0.0 { 1.0 + (factor_profundidad * 0.5) } else { 1.0 };
        self.motor.impulsar_y(fuerza / resistencia);
    }

    pub fn actualizar_visuales(&mut self, y: f32, dt: f32) {
        let direccion_deseada = if self.motor.velocidad_x >= 0.0 { 1.0 } else { -1.0 };
        self.volteo += (direccion_deseada - self.volteo) * dt * 5.0;

        let vel_total = (self.motor.velocidad_x.powi(2) + self.motor.velocidad_y.powi(2)).sqrt();
        self.angulo_helice += vel_total * dt * 0.1;

        let factor_profundidad = y / screen_height();
        self.estela.iter_mut().for_each(|p| {
            p.vida -= dt * 1.5;
            p.x += p.dx * dt;
            p.y += (p.dy - 40.0 * (1.0 + factor_profundidad)) * dt;
        });
        self.estela.retain(|p| p.vida > 0.0);
    }

    pub fn generar_estela(&mut self, x: f32, y: f32, cantidad: usize) {
        for _ in 0..cantidad {
            self.estela.push(Particula {
                x: x + rand::gen_range(0.0, self.tamano),
                y: y + rand::gen_range(0.0, self.tamano),
                dx: rand::gen_range(-30.0, 30.0),
                dy: rand::gen_range(-20.0, 20.0),
                vida: rand::gen_range(0.5, 1.0),
            });
        }
    }

    pub fn dibujar(&self, x: f32, y: f32) {
        for p in &self.estela {
            let color = Color::new(0.8, 0.9, 1.0, p.vida * 0.5);
            draw_circle(p.x, p.y, 3.5 * p.vida, color);
        }

        let radio = self.tamano * 0.4;
        let ancho_variable = self.tamano * self.volteo.abs();
        let center_x = x + self.tamano / 2.0;
        let start_x = center_x - (ancho_variable / 2.0);

        let helice_x = center_x - (self.tamano * 0.65 * self.volteo);
        let oscilacion = self.angulo_helice.sin() * 12.0;
        draw_line(helice_x, y + radio + oscilacion, helice_x, y + radio - oscilacion, 6.0, DARKBLUE);

        let turret_x = center_x + (self.tamano * 0.15 * self.volteo) - (self.tamano * 0.15);
        draw_rectangle(turret_x, y - self.tamano * 0.2, self.tamano * 0.3, self.tamano * 0.3, SKYBLUE);
        draw_rectangle(turret_x + self.tamano * 0.1, y - self.tamano * 0.4, self.tamano * 0.06, self.tamano * 0.25, GRAY);
        
        let lente_offset = if self.volteo >= 0.0 { 0.0 } else { -self.tamano * 0.1 };
        draw_rectangle(turret_x + self.tamano * 0.1 + lente_offset, y - self.tamano * 0.45, self.tamano * 0.15, self.tamano * 0.08, DARKGRAY);

        draw_circle(start_x, y + radio, radio, SKYBLUE);
        draw_circle(start_x + ancho_variable, y + radio, radio, SKYBLUE);
        draw_rectangle(start_x, y, ancho_variable, radio * 2.0, SKYBLUE);

        draw_rectangle(start_x, y, ancho_variable, 5.0, Color::new(0.8, 0.95, 1.0, 1.0));
        
        let ojo_x = center_x + (self.tamano * 0.35 * self.volteo);
        draw_circle(ojo_x, y + radio, self.tamano * 0.18, DARKBLUE);
        draw_circle(ojo_x, y + radio, self.tamano * 0.14, WHITE);
        draw_circle(ojo_x + (self.volteo * 2.0), y + radio, self.tamano * 0.07, BLACK);
    }
}
