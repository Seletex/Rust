use macroquad::prelude::*;


mod gravedad;
mod submarino;
mod jugador; 

// 2. Traemos el struct al alcance actual
use jugador::Jugador; 

#[macroquad::main("Control de Cuadro")]
async fn main() {
    let mut jugador = Jugador::new();

    loop {
        clear_background(BLACK);

        jugador.actualizar();
        jugador.dibujar();

        next_frame().await
    }
}