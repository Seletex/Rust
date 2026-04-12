use macroquad::prelude::*;

fn actualizar_estado(velocidad: f32, dt: f32, flotabilidad: f32) -> f32 {
    let gravedad = 9.81;
    let friccion = 10.0;
    let masa = 100.0;

    // F_net = Peso - Flotabilidad + Friccion (hacia abajo es positivo en pantalla)
    let peso = masa * gravedad;
    let sumatoria_fuerzas = peso - flotabilidad + (friccion * velocidad);
    let aceleracion = sumatoria_fuerzas / masa;

    let nueva_velocidad = velocidad + aceleracion * dt;
    nueva_velocidad
}

#[macroquad::main("Submarino")]
async fn main() {
    // Inicializamos el submarino en el centro de la pantalla
    let mut velocidad = 0.0;
    let mut posicion = screen_height() / 2.0;
    let mut flotabilidad = 1000.0; // Valor base (cercano al peso para equilibrio)

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // Dibujar círculo de referencia en el centro
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        draw_circle_lines(cx, cy, 30.0, 1.0, BLUE);

        // Controles para cambiar la flotabilidad (llenar/vaciar tanques)
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            flotabilidad += 200.0 * dt;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            flotabilidad -= 200.0 * dt;
        }

        // Llamamos a la función física que definimos arriba
        velocidad = actualizar_estado(velocidad, dt, flotabilidad);
        posicion = posicion + velocidad * dt;
        // Dibujar el submarino (un círculo gris oscuro)
        let x = screen_width() / 2.0;
        let y = posicion;
        draw_circle(x, y, 20.0, BLUE);
        next_frame().await;
    }
}
