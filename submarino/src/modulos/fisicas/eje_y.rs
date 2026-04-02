pub struct Estado {
    pub posicion: f32,
    pub velocidad: f32,
}

pub fn actualizar_estado(estado: Estado, dt: f32, flotabilidad: f32) -> Estado {
    let gravedad = 9.81;
    let friccion = 0.5; 
    let masa = 100.0;

    // F_net = Peso - Flotabilidad + Friccion (sentido hacia abajo es positivo en pantalla)
    // Peso = masa * gravedad
    let peso = masa * gravedad;
    let sumatoria_fuerzas = peso - flotabilidad + (friccion * estado.velocidad);
    let aceleracion = sumatoria_fuerzas / masa;
    
    let nueva_velocidad = estado.velocidad + aceleracion * dt;
    let nueva_posicion = estado.posicion + nueva_velocidad * dt;

    Estado {
        posicion: nueva_posicion,
        velocidad: nueva_velocidad,
    }
}
