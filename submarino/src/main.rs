mod modulos;
use modulos::fisicas::eje_y::FisicasEjeY;

fn main() {
    println!("Iniciando simulación del submarino...");
    
    // Suponiendo que la pantalla tiene 500 de alto, la mitad es 250
    let mut submarino_y = FisicasEjeY::new(250.0);
    
    println!("Posición inicial: {}", submarino_y.posicion);
    
    // Ejecutamos 10 pasos de simulación para ver los cálculos
    for indice in 1..=10 {
        submarino_y.actualizar();
    }
    
    println!("\nSimulación finalizada.");
}
