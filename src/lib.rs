mod physics;
mod player;

use bevy::prelude::*;
use physics::{Collider, PhysicsPlugin, Solid};
use player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    #[default]
    // During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugin(PlayerPlugin);
        app.add_plugin(PhysicsPlugin);

        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Solid,
        Collider {
            size: Vec2::new(100.0, 100.0),
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Solid,
        Collider {
            size: Vec2::new(100.0, 50.0),
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(100.0, 50.0)),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Solid,
        Collider {
            size: Vec2::new(400.0, 32.0),
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(400.0, 32.0)),
                ..default()
            },
            ..default()
        },
    ));
}
