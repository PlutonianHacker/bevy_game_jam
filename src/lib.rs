mod physics;
mod player;
mod level;
mod loader;

use bevy::prelude::*;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use level::LevelPlugin;
use loader::LoadingPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    Transitioning,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugin(PlayerPlugin);
        app.add_plugin(PhysicsPlugin);
        app.add_plugin(LoadingPlugin);
        app.add_plugin(LevelPlugin);

        app.add_system(setup.in_schedule(OnEnter(GameState::Loading)));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
