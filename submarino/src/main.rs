use bevy::prelude::*;

// 1. Definimos un componente personalizado, Macroprocedural 
//#[derive(Component)]` indica que esta estructura puede ser usada como un componente en Bevy.
#[derive(Component)]
struct Posicion {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_systems(Startup, crear_entidad)
        .add_systems(Update, leer_posicion)
        .run();
}

// 2. Sistema que crea (spawnea) una entidad con el componente
// spawn`Posicion`. `Commands` es un recurso que nos permite crear y manipular entidades en Bevy.

fn crear_entidad(mut commands: Commands) {
    commands.spawn(Posicion { x: 10.0, y: 20.0 });
    commands.spawn(Posicion { x: 30.0, y: 40.0 });
}

// 3. Sistema que busca entidades que tengan el componente 'Posicion' y lee sus datos
// `Query<&Posicion>` indica que queremos acceder a todas las entidades que tengan el componente `Posicion`.
// `&Posicion` significa que estamos obteniendo una referencia al componente, no una copia.
// `for pos in &query` itera sobre todas las entidades que tienen el componente `Posicion`.
// Entrega de daos a bevy para que pueda procesar y renderizar la información de las entidades.
fn leer_posicion(query: Query<&Posicion>) {
    for pos in &query {
        println!("La posición es: ({}, {})", pos.x, pos.y);
    }
}