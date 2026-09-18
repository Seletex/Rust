// ============================================================================
// WORLD - Sistema principal de gameplay (mundo, entidades, colisiones)
// ============================================================================
// Todo el gameplay corre en un ÚNICO sistema (detectar_colisiones) para evitar
// conflictos de queries mutables entre sistemas paralelos (error B0001).
// Orden de ejecución por frame:
// 1. Input horizontal → vel.x
// 2. Gravedad → vel.y
// 3. Salto (check en_suelo ANTES de resetear)
// 4. Colisión AABB obstáculos (push horizontal, aterrizar, golpear cabeza)
// 4. Suelo mundo + límites horizontales
// 5. Toro: perseguir + mover + colisión GameOver
// 6. Meta → Victoria
// 7. Aplicar velocidad + sync Transform

use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::{ConfigJuego, MeshHandles, SpriteHandles};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup: cargar recursos (mallas, materiales) → spawnear entidades (encadenados)
            .add_systems(Startup, (cargar_recursos, inicializar_entidades).chain())
            // Al entrar a Jugando: resetear posiciones
            .add_systems(OnEnter(GameState::Jugando), reiniciar_posiciones)
            // Cada frame en estado Jugando: detección colisiones + gameplay
            .add_systems(Update, detectar_colisiones.run_if(in_state(GameState::Jugando)))
            // Cada frame en estado Jugando: procesar eventos → cambiar estados
            .add_systems(Update, procesar_eventos_partida.run_if(in_state(GameState::Jugando)));
    }
}

// --- Startup: Carga de assets procedimentales ---
fn cargar_recursos(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Geometría: cuadrado 64x64 para entidades, rectángulo 800x600 para fondo
    let malla_cuadrado = meshes.add(Rectangle::new(64.0, 64.0));
    let malla_fondo = meshes.add(Rectangle::new(800.0, 600.0));

    // Materiales de color (sprites procedimentales)
    let material_jugador = materials.add(Color::srgb(0.2, 0.6, 1.0));   // Azul
    let material_toro = materials.add(Color::srgb(0.9, 0.2, 0.2));       // Rojo
    let material_obstaculo = materials.add(Color::srgb(0.5, 0.5, 0.5)); // Gris
    let material_meta = materials.add(Color::srgb(0.2, 0.8, 0.3));      // Verde
    let material_fondo = materials.add(Color::srgb(0.1, 0.1, 0.15));    // Gris oscuro

    // Inserta handles de mallas para reutilizar
    commands.insert_resource(MeshHandles {
        cuadrado: malla_cuadrado,
        fondo: malla_fondo,
    });

    // Inserta handles de materiales para reutilizar
    commands.insert_resource(SpriteHandles {
        jugador: material_jugador,
        toro: material_toro,
        obstaculo: material_obstaculo,
        meta: material_meta,
        fondo: material_fondo,
    });
}

// --- Startup: Spawneo de entidades iniciales ---
fn inicializar_entidades(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    sprite_handles: Res<SpriteHandles>,
) {
    // Cámara 2D
    commands.spawn(Camera2d);

    // Fondo: rectángulo 800x600 color gris oscuro, z = -1 (detrás de todo)
    commands.spawn((
        Mesh2d(mesh_handles.fondo.clone()),
        MeshMaterial2d(sprite_handles.fondo.clone()),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Jugador: cuadrado azul, posición inicial (-250, -200)
    commands.spawn((
        Mesh2d(mesh_handles.cuadrado.clone()),
        MeshMaterial2d(sprite_handles.jugador.clone()),
        Transform::from_xyz(-250.0, -200.0, 0.0),
        Posicion { x: -250.0, y: -200.0 },
        Velocidad { x: 0.0, y: 0.0 },
        Jugador,
        EnSuelo(true),  // Empieza en suelo para poder saltar
        SpriteSize { width: 64.0, height: 64.0 },
    ));

    // Toro: cuadrado rojo, posición inicial (-380, -180) - lejos a la izquierda
    commands.spawn((
        Mesh2d(mesh_handles.cuadrado.clone()),
        MeshMaterial2d(sprite_handles.toro.clone()),
        Transform::from_xyz(-380.0, -180.0, 0.0),
        Posicion { x: -380.0, y: -180.0 },
        Velocidad { x: 0.0, y: 0.0 },
        Toro,
        SpriteSize { width: 64.0, height: 64.0 },
    ));

    // Obstáculos: 4 plataformas grises (x, y) - bajos para ser alcanzables
    for (x, y) in [(50.0, -200.0), (150.0, -180.0), (-100.0, -220.0), (250.0, -240.0)] {
        commands.spawn((
            Mesh2d(mesh_handles.cuadrado.clone()),
            MeshMaterial2d(sprite_handles.obstaculo.clone()),
            Transform::from_xyz(x, y, 0.0),
            Posicion { x, y },
            Obstaculo,
            SpriteSize { width: 64.0, height: 64.0 },
        ));
    }

    // Meta (refugio): cuadrado verde a la derecha (300, -200)
    commands.spawn((
        Mesh2d(mesh_handles.cuadrado.clone()),
        MeshMaterial2d(sprite_handles.meta.clone()),
        Transform::from_xyz(300.0, -200.0, 0.0),
        Posicion { x: 300.0, y: -200.0 },
        Meta,
        SpriteSize { width: 64.0, height: 64.0 },
    ));

    info!("Entidades inicializadas: Jugador, Toro, Obstáculos, Meta");
}

// --- OnEnter(Jugando): Reset posiciones al iniciar/reiniciar partida ---
fn reiniciar_posiciones(
    mut q_jugador: Query<(&mut Posicion, &mut Velocidad, &mut EnSuelo), With<Jugador>>,
    mut q_toro: Query<(&mut Posicion, &mut Velocidad), (With<Toro>, Without<Jugador>)>,
) {
    if let Ok((mut pos, mut vel, mut en_suelo)) = q_jugador.single_mut() {
        pos.x = -250.0; pos.y = -200.0;
        vel.x = 0.0; vel.y = 0.0;
        en_suelo.0 = true;
    }
    if let Ok((mut pos, mut vel)) = q_toro.single_mut() {
        pos.x = -380.0; pos.y = -180.0;
        vel.x = 0.0; vel.y = 0.0;
    }
}

// ============================================================================
// SISTEMA PRINCIPAL: detectar_colisiones
// Todo el gameplay en un sistema para evitar conflictos de queries (B0001)
// ============================================================================
pub fn detectar_colisiones(
    teclado: Res<ButtonInput<KeyCode>>,           // Input teclado
    time: Res<Time>,                               // Delta time
    config: Res<ConfigJuego>,                      // Config velocidades, límites
    // Jugador: posición, velocidad, estado suelo, transform (mutable)
    mut jugador_query: Query<
        (Entity, &mut Posicion, &mut Velocidad, &mut EnSuelo, Option<&mut Transform>),
        (With<Jugador>, Without<Toro>)
    >,
    // Obstáculos: solo posición (inmutables)
    obstaculos_query: Query<(Entity, &Posicion), (With<Obstaculo>, Without<Jugador>, Without<Toro>)>,
    // Toro: posición, velocidad, transform (mutable) - para moverlo
    mut toro_query: Query<
        (Entity, &mut Posicion, &mut Velocidad, Option<&mut Transform>),
        (With<Toro>, Without<Jugador>)
    >,
    // Meta: solo posición
    meta_query: Query<&Posicion, (With<Meta>, Without<Jugador>, Without<Toro>)>,
    // Writers para emitir eventos
    mut ev_salto: MessageWriter<EventoSaltoJugador>,
    mut ev_toro: MessageWriter<EventoToroAtrapaJugador>,
    mut ev_meta: MessageWriter<EventoLlegadaMeta>,
) {
    // Obtiene datos mutables del jugador único
    if let Ok((_jugador_entity, mut pos_j, mut vel_j, mut en_suelo, transform)) = jugador_query.single_mut() {
        let half_size = 32.0;       // Radio jugador (64/2)
        let obstaculo_half = 32.0;  // Radio obstáculo (64/2)
        
        // --- 1. INPUT HORIZONTAL ---
        let mut direccion = 0.0;
        if teclado.pressed(KeyCode::KeyD) || teclado.pressed(KeyCode::ArrowRight) { direccion += 1.0; }
        if teclado.pressed(KeyCode::KeyA) || teclado.pressed(KeyCode::ArrowLeft)  { direccion -= 1.0; }
        vel_j.x = direccion * config.velocidad_jugador;

        // --- 2. GRAVEDAD ---
        vel_j.y -= config.gravedad * time.delta_secs();
        
        // --- 3. SALTO (check en_suelo ANTES de resetear a false) ---
        if teclado.just_pressed(KeyCode::Space) && en_suelo.0 {
            vel_j.y = config.fuerza_salto;
            ev_salto.write(EventoSaltoJugador);  // Emite evento → Audio FX
        }
        
        // Reset flag suelo (se pondrá true si aterriza en colisión/suelo)
        en_suelo.0 = false;

        // --- 4. COLISIÓN AABB CON OBSTÁCULOS (plataformas sólidas) ---
        for (_obs_entity, pos_o) in obstaculos_query.iter() {
            let dx = pos_j.x - pos_o.x;
            let dy = pos_j.y - pos_o.y;
            let overlap_x = half_size + obstaculo_half - dx.abs();
            let overlap_y = half_size + obstaculo_half - dy.abs();

            if overlap_x > 0.0 && overlap_y > 0.0 {  // Hay solapamiento
                if overlap_x < overlap_y {
                    // COLISIÓN HORIZONTAL (lados) → push + vel.x=0
                    if dx > 0.0 { pos_j.x = pos_o.x + half_size + obstaculo_half; }
                    else { pos_j.x = pos_o.x - half_size - obstaculo_half; }
                    vel_j.x = 0.0;
                } else {
                    // COLISIÓN VERTICAL
                    if dy > 0.0 {
                        // Jugador ARRIBA del obstáculo (aterriza)
                        pos_j.y = pos_o.y + half_size + obstaculo_half;
                        vel_j.y = 0.0;
                        en_suelo.0 = true;  // Puede saltar de nuevo
                    } else {
                        // Jugador ABAJO (golpea con la cabeza)
                        pos_j.y = pos_o.y - half_size - obstaculo_half;
                        vel_j.y = 0.0;
                    }
                }
            }
        }

        // --- 5. SUELO MUNDO (fondo de pantalla) ---
        let ground_y = -config.limite_y + half_size;
        if pos_j.y <= ground_y {
            pos_j.y = ground_y;
            if vel_j.y < 0.0 { vel_j.y = 0.0; en_suelo.0 = true; }
        }

        // --- 6. LÍMITES HORIZONTALES ---
        pos_j.x = pos_j.x.clamp(-config.limite_x + half_size, config.limite_x - half_size);

        // --- 7. TORO: PERSEGUIR, MOVER, COLISIÓN GAMEOVER ---
        let dt = time.delta_secs();
        for (_toro_entity, mut pos_t, mut vel_t, toro_transform) in toro_query.iter_mut() {
            let dx = pos_j.x - pos_t.x;
            let dy = pos_j.y - pos_t.y;
            let distancia = (dx * dx + dy * dy).sqrt();

            if distancia > 0.0 {
                // Vector dirección normalizado × velocidad toro
                vel_t.x = (dx / distancia) * config.velocidad_toro;
                vel_t.y = (dy / distancia) * config.velocidad_toro;
            }

            // Mover toro
            pos_t.x += vel_t.x * dt;
            pos_t.y += vel_t.y * dt;

            if let Some(mut t) = toro_transform {
                t.translation.x = pos_t.x;
                t.translation.y = pos_t.y;
            }

            // Colisión toro-jugador (GameOver)
            let dist = ((pos_j.x - pos_t.x).powi(2) + (pos_j.y - pos_t.y).powi(2)).sqrt();
            if dist < config.umbral_colision {
                ev_toro.write(EventoToroAtrapaJugador);  // → Audio FX + State GameOver
                return;
            }
        }

        // --- 8. META (VICTORIA) ---
        for pos_m in meta_query.iter() {
            let dist = ((pos_j.x - pos_m.x).powi(2) + (pos_j.y - pos_m.y).powi(2)).sqrt();
            if dist < config.umbral_colision {
                ev_meta.write(EventoLlegadaMeta);  // → Audio FX + State Victoria
                return;
            }
        }

        // --- 9. APLICAR VELOCIDAD JUGADOR + SYNC TRANSFORM ---
        let dt = time.delta_secs();
        pos_j.x += vel_j.x * dt;
        pos_j.y += vel_j.y * dt;

        if let Some(mut transform) = transform {
            transform.translation.x = pos_j.x;
            transform.translation.y = pos_j.y;
        }
    }
}

// --- Procesa eventos → cambia estados ---
fn procesar_eventos_partida(
    mut ev_toro: MessageReader<EventoToroAtrapaJugador>,
    mut ev_meta: MessageReader<EventoLlegadaMeta>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_toro.read() {
        warn!("¡El toro te atrapó!");
        next_state.set(GameState::GameOver);
    }
    for _ in ev_meta.read() {
        info!("¡Llegaste a la meta!");
        next_state.set(GameState::Victoria);
    }
}