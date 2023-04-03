use bevy::{
    prelude::*,
    sprite::collide_aabb::{self},
};

use crate::{
    player::{self, Velocity},
    GameState,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_gravity, update)
                .chain()
                .after(player::handle_input)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

fn apply_gravity(mut actors: Query<&mut Velocity, With<Actor>>) {
    for mut velocity in &mut actors {
        
        //velocity.y = (velocity.y - 1.0);//.max(-32.0);
    }
}

fn update(
    mut actors: Query<(&Velocity, &mut Transform, &Collider), (With<Actor>, Without<Solid>)>,
    solids: Query<(&Transform, &Collider), (Without<Actor>, With<Solid>)>,
) {
    for (velocity, mut actor_transform, actor_collider) in &mut actors {
        actor_transform.translation.x += velocity.x;

        for (transform, collider) in &solids {
            if let Some(_) = collide_aabb::collide(
                actor_transform.translation,
                actor_collider.size,
                transform.translation,
                collider.size,
            ) {
                let actor_pos = actor_transform.translation;
                let actor_size = actor_collider.size;

                let solid_pos = transform.translation;
                let solid_size = collider.size;

                let amount = overlap(
                    actor_pos.x - (actor_size.x / 2.0),
                    actor_pos.x + (actor_size.x / 2.0),
                    solid_pos.x - (solid_size.x / 2.0),
                    solid_pos.x + (solid_size.x / 2.0),
                );

                let sign = velocity.x.signum();

                actor_transform.translation.x += amount * -sign;
            }
        }

        actor_transform.translation.y += velocity.y;

        for (transform, collider) in &solids {
            if let Some(_) = collide_aabb::collide(
                actor_transform.translation,
                actor_collider.size,
                transform.translation,
                collider.size,
            ) {
                let actor_pos = actor_transform.translation;
                let actor_size = actor_collider.size;

                let solid_pos = transform.translation;
                let solid_size = collider.size;

                let amount = overlap(
                    actor_pos.y - (actor_size.y / 2.0),
                    actor_pos.y + (actor_size.y / 2.0),
                    solid_pos.y - (solid_size.y / 2.0),
                    solid_pos.y + (solid_size.y / 2.0),
                );

                let sign = velocity.y.signum();

                actor_transform.translation.y += amount * -sign;
            }
        }
    }
}

fn overlap(min_1: f32, max_1: f32, min_2: f32, max_2: f32) -> f32 {
    (0.0_f32).max((max_1).min(max_2) - (min_1).max(min_2))
}
