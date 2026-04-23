use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Submarino {
    posicion: f32,
    velocidad: f32,
    flotabilidad: f32,
}
struct Configuracion {
    gravedad: f32,
    friccion: f32,
    masa: f32,
}
#[derive(Clone, Copy)]
enum Orden {
    Subir,
    Bajar,
    Nada,
}
fn controles(submarino: Submarino, orden: Orden, dt: f32) -> Submarino {
    let mut flotabilidad = submarino.flotabilidad;
    match orden {
        Orden::Subir => flotabilidad += 200.0 * dt,
        Orden::Bajar => flotabilidad -= 200.0 * dt,
        Orden::Nada => {}
    }
    Submarino {
        posicion: submarino.posicion,
        velocidad: submarino.velocidad,
        flotabilidad: flotabilidad,
    }
}
fn actualizar_estado<T>(
    estado: Submarino,
    dt: f32,
    configuracion: &Configuracion,
    calcular_fuerzas: T,
) -> Submarino
where
    T: Fn(&Submarino, &Configuracion) -> f32,
{
    let fuerza_neta = calcular_fuerzas(&estado, configuracion);
    let aceleracion = fuerza_neta / configuracion.masa;

    let nueva_velocidad = estado.velocidad + aceleracion * dt;
    let nueva_posicion = estado.posicion + nueva_velocidad * dt;
    Submarino {
        posicion: nueva_posicion,
        velocidad: nueva_velocidad,
        flotabilidad: estado.flotabilidad,
    }
}

#[macroquad::main("Submarino")]
async fn main() {
    // Inicializamos el submarino en el centro de la pantalla
    let mut submarino = Submarino {
        posicion: screen_height() / 2.0,
        velocidad: 0.0,
        flotabilidad: 1000.0, // Valor base (cercano al peso para equilibrio)
    };
    let configuracion = Configuracion {
        gravedad: 9.81,
        friccion: 20.0,
        masa: 100.0,
    };
    let mut simulacion = std::iter::repeat(()).scan(submarino, |estado, _| {
        let dt = get_frame_time();
        let orden = if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            Orden::Bajar
        } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            Orden::Subir
        } else {
            Orden::Nada
        };
        *estado = controles(*estado, orden, dt);
        *estado = actualizar_estado(*estado, dt, &configuracion, |submarino, configuracion| {
            [
                configuracion.masa * configuracion.gravedad,
                -submarino.flotabilidad,
                configuracion.friccion * submarino.velocidad,
            ]
            .into_iter()
            .map(|f| f * 1.0)
            .sum()
        });
        Some(*estado)
    });

    loop {
        clear_background(BLACK);

        if let Some(estado) = simulacion.next() {
            submarino = estado;
        }
        // Dibujar círculo de referencia en el centro
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        draw_circle_lines(cx, cy, 30.0, 1.0, BLUE);
        // Controles para cambiar la flotabilidad (llenar/vaciar tanques)

        // Dibujar el submarino (un círculo gris oscuro)
        let x = screen_width() / 2.0;
        let y = submarino.posicion;
        draw_circle(x, y, 20.0, BLUE);

        next_frame().await;
    }
}
