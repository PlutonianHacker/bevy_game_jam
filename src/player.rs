use bevy::prelude::*;

use crate::{GameState, physics::{Actor, Collider}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(handle_input.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    actor: Actor,
    velocity: Velocity,
    acceleration: Acceleration,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            actor: Actor,
            collider: Collider {
                size: Vec2::new(32.0, 32.0),
            },
            sprite: SpriteBundle::default(),
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

pub(crate) fn handle_input(input: Res<Input<KeyCode>>, mut player: Query<&mut Velocity, With<Player>>) {
    let mut velocity = player.single_mut();

    if input.pressed(KeyCode::A) {
        velocity.x = (velocity.x - 1.0).max(-12.0);
    } else if input.pressed(KeyCode::D) {
        velocity.x = (velocity.x + 1.0).min(12.0);
    } else {
        velocity.x = 0.0;
    }

    
    if input.pressed(KeyCode::W) {
        velocity.y = (velocity.y + 1.0).min(32.0);
    } else if input.pressed(KeyCode::S) {
        velocity.y = (velocity.y - 1.0).max(-32.0);
    } else {
        velocity.y = 0.0;
    }
    //println!("({}, {})", velocity.x, velocity.y);
}
