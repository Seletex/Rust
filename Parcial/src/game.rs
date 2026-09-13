use bevy::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::player::*;
use crate::audio::cargar_audios;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (inicializar_entidades, cargar_audios))
            .add_systems(OnEnter(GameState::Jugando), reiniciar_posiciones)
            .add_systems(
                Update,
                esperar_inicio_juego.run_if(in_state(GameState::Inicio)),
            )
            .add_systems(
                Update,
                esperar_reinicio_juego.run_if(
                    in_state(GameState::Victoria).or_else(in_state(GameState::Derrota)),
                ),
            )
            .add_systems(
                Update,
                (
                    controlar_jugador,
                    perseguir_jugador,
                    aplicar_velocidad,
                )
                    .run_if(in_state(GameState::Jugando)),
            )
            .add_systems(
                PostUpdate,
                (
                    validar_limites,
                    detectar_colisiones,
                    procesar_eventos_partida,
                )
                    .run_if(in_state(GameState::Jugando)),
            );
    }
}

fn inicializar_entidades(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let malla_circulo = meshes.add(Circle::new(16.0));
    let malla_cuadrado = meshes.add(Rectangle::new(32.0, 32.0));
    let malla_triangulo = meshes.add(RegularPolygon::new(22.0, 3));

    let material_jugador = materials.add(Color::srgb(0.2, 0.6, 1.0));
    let material_toro = materials.add(Color::srgb(0.9, 0.2, 0.2));
    let material_obstaculo = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let material_meta = materials.add(Color::srgb(0.2, 0.8, 0.3));

    // Jugador
    commands.spawn((
        Mesh2d(malla_circulo.clone()),
        MeshMaterial2d(material_jugador),
        Transform::from_xyz(-250.0, 0.0, 0.0),
        Posicion { x: -250.0, y: 0.0 },
        Velocidad { x: 0.0, y: 0.0 },
        Jugador,
    ));

    // Toro
    commands.spawn((
        Mesh2d(malla_triangulo),
        MeshMaterial2d(material_toro),
        Transform::from_xyz(-380.0, -180.0, 0.0),
        Posicion {
            x: -380.0,
            y: -180.0,
        },
        Velocidad { x: 0.0, y: 0.0 },
        Toro,
    ));

    // Obstáculos
    for (x, y) in [(50.0, 120.0), (100.0, -120.0)] {
        commands.spawn((
            Mesh2d(malla_cuadrado.clone()),
            MeshMaterial2d(material_obstaculo.clone()),
            Transform::from_xyz(x, y, 0.0),
            Posicion { x, y },
            Obstaculo,
        ));
    }

    // Meta
    commands.spawn((
        Mesh2d(malla_circulo),
        MeshMaterial2d(material_meta),
        Transform::from_xyz(300.0, 0.0, 0.0),
        Posicion { x: 300.0, y: 0.0 },
        Meta,
    ));

    println!("JUEGO LISTO: Presiona ESPACIO para iniciar...");
}

fn esperar_inicio_juego(
    teclado: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if teclado.just_pressed(KeyCode::Space) {
        println!("Iniciando partida...");
        next_state.set(GameState::Jugando);
    }
}

fn esperar_reinicio_juego(
    teclado: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if teclado.just_pressed(KeyCode::Space) {
        println!("Reiniciando partida...");
        next_state.set(GameState::Jugando);
    }
}

fn reiniciar_posiciones(
    mut q_jugador: Query<(&mut Posicion, &mut Velocidad), With<Jugador>>,
    mut q_toro: Query<(&mut Posicion, &mut Velocidad), (With<Toro>, Without<Jugador>)>,
) {
    if let Ok((mut pos, mut vel)) = q_jugador.single_mut() {
        pos.x = -250.0;
        pos.y = 0.0;
        vel.x = 0.0;
        vel.y = 0.0;
    }
    if let Ok((mut pos, mut vel)) = q_toro.single_mut() {
        pos.x = -380.0;
        pos.y = -180.0;
        vel.x = 0.0;
        vel.y = 0.0;
    }
}

fn detectar_colisiones(
    jugador_query: Query<&Posicion, With<Jugador>>,
    obstaculos_query: Query<&Posicion, (With<Obstaculo>, Without<Jugador>)>,
    toro_query: Query<&Posicion, (With<Toro>, Without<Jugador>)>,
    meta_query: Query<&Posicion, With<Meta>>,
    mut ev_perder: MessageWriter<EventoJuegoPerdido>,
    mut ev_ganar: MessageWriter<EventoJuegoGanado>,
) {
    if let Ok(pos_j) = jugador_query.single() {
        let umbral_colision = 25.0;

        for pos_o in &obstaculos_query {
            let dist = ((pos_j.x - pos_o.x).powi(2) + (pos_j.y - pos_o.y).powi(2)).sqrt();
            if dist < umbral_colision {
                ev_perder.write(EventoJuegoPerdido);
                return;
            }
        }

        for pos_t in &toro_query {
            let dist = ((pos_j.x - pos_t.x).powi(2) + (pos_j.y - pos_t.y).powi(2)).sqrt();
            if dist < umbral_colision {
                ev_perder.write(EventoJuegoPerdido);
                return;
            }
        }

        for pos_m in &meta_query {
            let dist = ((pos_j.x - pos_m.x).powi(2) + (pos_j.y - pos_m.y).powi(2)).sqrt();
            if dist < umbral_colision {
                ev_ganar.write(EventoJuegoGanado);
                return;
            }
        }
    }
}

fn procesar_eventos_partida(
    mut ev_perder: MessageReader<EventoJuegoPerdido>,
    mut ev_ganar: MessageReader<EventoJuegoGanado>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_perder.read() {
        println!("¡GAME OVER! Has chocado contra un obstáculo o el toro te atrapó.");
        next_state.set(GameState::Derrota);
    }

    for _ in ev_ganar.read() {
        println!("¡VICTORIA! Alcanzaste el refugio seguro.");
        next_state.set(GameState::Victoria);
    }
} // <--- Asegúrate de que esta llave exista al final del archivo