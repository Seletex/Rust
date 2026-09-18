use bevy::prelude::*;
use crate::events::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), spawn_menu_ui)
            .add_systems(OnExit(GameState::Menu), despawn_menu_ui)
            .add_systems(OnEnter(GameState::GameOver), spawn_gameover_ui)
            .add_systems(OnExit(GameState::GameOver), despawn_gameover_ui)
            .add_systems(OnEnter(GameState::Victoria), spawn_victoria_ui)
            .add_systems(OnExit(GameState::Victoria), despawn_victoria_ui)
            .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(Update, handle_gameover_input.run_if(in_state(GameState::GameOver).or_else(in_state(GameState::Victoria))));
    }
}

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct VictoriaUI;

fn spawn_menu_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        MenuUI,
        BackgroundColor(Color::NONE),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("EXTREME PAMPLONA"),
            TextFont {
                font_size: FontSize::Px(60.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.0)),
        ));
        parent.spawn((
            Text::new("Presiona ESPACIO para iniciar"),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("Evita al toro y llega a la meta"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

fn despawn_menu_ui(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_gameover_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        GameOverUI,
        BackgroundColor(Color::NONE),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: FontSize::Px(60.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
        ));
        parent.spawn((
            Text::new("El toro te atrapó!"),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("Presiona ESPACIO para reiniciar"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

fn despawn_gameover_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_victoria_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        VictoriaUI,
        BackgroundColor(Color::NONE),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("¡VICTORIA!"),
            TextFont {
                font_size: FontSize::Px(60.0),
                ..default()
            },
            TextColor(Color::srgb(0.2, 1.0, 0.3)),
        ));
        parent.spawn((
            Text::new("Llegaste a salvo a la meta"),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("Presiona ESPACIO para jugar de nuevo"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

fn despawn_victoria_ui(mut commands: Commands, query: Query<Entity, With<VictoriaUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_input(
    teclado: Res<ButtonInput<KeyCode>>,
    mut ev_inicio: MessageWriter<EventoJuegoIniciado>,
) {
    if teclado.just_pressed(KeyCode::Space) || teclado.just_pressed(KeyCode::Enter) {
        info!("🚀 ESPACIO/ENTER presionado - Iniciando juego");
        ev_inicio.write(EventoJuegoIniciado);
    }
}

fn handle_gameover_input(
    teclado: Res<ButtonInput<KeyCode>>,
    mut ev_reinicio: MessageWriter<EventoJuegoReiniciado>,
) {
    if teclado.just_pressed(KeyCode::Space) || teclado.just_pressed(KeyCode::Enter) {
        ev_reinicio.write(EventoJuegoReiniciado);
    }
}