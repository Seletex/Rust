use bevy::prelude::*;
use crate::components::*;

pub fn inicializar_entidades_y_audios(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // =========================================================================
    // CARGA DE RECURSOS DE AUDIO DESDE LA CARPETA ASSETS
    // =========================================================================

    let handle_menu: Handle<AudioSource> = asset_server.load("audio/inicio.mp3");
    let handle_juego: Handle<AudioSource> = asset_server.load("audio/fondo.mp3");
    let handle_victoria: Handle<AudioSource> = asset_server.load("audio/Victoria.mp3");
    let handle_derrota: Handle<AudioSource> = asset_server.load("audio/Perder.mp3");
    let handle_pasos: Handle<AudioSource> = asset_server.load("audio/Movimiento.mp3");

    commands.insert_resource(ColeccionAudioProcedural {
        menu: handle_menu.clone(),
        juego: handle_juego,
        victoria: handle_victoria,
        derrota: handle_derrota,
        pasos: handle_pasos,
    });

    commands.spawn((
        AudioPlayer::new(handle_menu),
        PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.2)),
        MusicaFondo,
    ));

    // =========================================================================
    // GEOMETRÍAS Y ENTIDADES DEL MAPA
    // =========================================================================

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