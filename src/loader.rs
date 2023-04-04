use std::ops::{Deref, DerefMut};

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use serde::Deserialize;
use serde_json::Value;

use crate::GameState;

/// Track loading state and prevent unused loaded handles
/// from being dropped.
#[derive(Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-dead-ed40-a018b74b5052"]
pub struct JsonFile(Value);

impl Deref for JsonFile {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsonFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
struct JsonFileLoader;

impl AssetLoader for JsonFileLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let content = serde_json::from_slice::<Value>(bytes)?;
            let json = JsonFile(content);
            load_context.set_default_asset(LoadedAsset::new(json));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<JsonFile>()
            .init_asset_loader::<JsonFileLoader>()
            .add_system(setup.in_schedule(OnEnter(GameState::Loading)))
            .add_system(update.in_set(OnUpdate(GameState::Loading)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut assets = vec![];

    let paths = ["levels", "textures", "tilesets"];

    for path in paths {
        assets.extend(
            asset_server
                .load_folder(path)
                .expect("Failed to load folder `levels`"),
        );
    }

    assets.push(asset_server.load_untyped("levels.json"));

    commands.insert_resource(AssetsLoading(assets));
}

fn update(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    use bevy::asset::LoadState;

    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready
            info!("Starting game");
            next_state.set(GameState::Playing);
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
