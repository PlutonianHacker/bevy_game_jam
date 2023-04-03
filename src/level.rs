use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;
use serde_json::Value;

use crate::{loader::JsonFile, GameState};

/// A tiled map.
#[derive(Deserialize)]
pub struct TiledMap {
    pub height: u32,
    pub width: u32,

    pub layers: Vec<TiledLayer>,

    #[serde(rename = "nextlayerid")]
    pub next_layer_id: u32,
    #[serde(rename = "nextobjectid")]
    pub next_object_id: u32,

    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,

    pub tilesets: Vec<Tileset>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Tileset {
    // Embedded,
    External {
        #[serde(rename = "firstgid")]
        first_gid: u32,
        source: String,
    },
}

/// A layer in a tile map.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum TiledLayer {
    TileLayer(TiledTileLayer),
    // ObjectLayer,
}

#[derive(Deserialize)]
pub struct TiledTileLayer {
    pub id: u32,
    pub name: String,

    pub data: Vec<u32>,

    pub height: u32,
    pub width: u32,

    pub opacity: u32,
    pub visible: bool,

    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize)]
pub struct TiledTileset {
    pub name: String,

    pub columns: u32,

    pub image: String,
    #[serde(rename = "imageheight")]
    pub image_height: u32,
    #[serde(rename = "imagewidth")]
    pub image_width: u32,

    pub margin: u32,
    pub spacing: u32,

    #[serde(rename = "tilecount")]
    pub tile_count: u32,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,

    #[serde(default)]
    pub tiles: Vec<TileProperties>,
}

#[derive(Deserialize, Clone)]
pub struct TileProperties {
    pub id: u32,
    pub properties: Vec<Property>,
}

#[derive(Deserialize, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

struct PropertyMap(HashMap<u32, TileProperties>);

impl PropertyMap {
    pub fn new(tileset: &TiledTileset) -> Self {
        let mut map = HashMap::new();

        for tile in tileset.tiles.iter() {
            let id = tile.id;

            map.insert(id, tile.clone());
        }

        Self(map)
    }
}

#[derive(Deserialize, Clone)]
pub struct LevelData {
    name: String,
    map: String,
    next: Option<String>,
}

#[derive(Resource, Deserialize)]
pub struct Levels(Vec<LevelData>);

impl Levels {
    fn get(&self, name: &String) -> &LevelData {
        self.0.iter().find(|data| &data.name == name).unwrap()
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
pub struct CurrentLevel(LevelData);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_levels.in_schedule(OnExit(GameState::Loading)))
            .add_system(spawn_map.in_schedule(OnEnter(GameState::Playing)))
            .add_system(test.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_map.in_schedule(OnEnter(GameState::Transitioning)))
            .add_system(level_transition.in_set(OnUpdate(GameState::Transitioning)));
    }
}

fn test(keyboard: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_released(KeyCode::T) {
        next_state.set(GameState::Transitioning);
    }
}

pub fn setup_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut json_files: ResMut<Assets<JsonFile>>,
) {
    let handle: Handle<JsonFile> = asset_server.get_handle("levels.json");
    let json = json_files
        .get_mut(&handle)
        .expect("Failed to get JSON file `levels.json`");
    // Should only have to create this once after loading so its okay to
    // take the file data.
    let levels = serde_json::from_value::<Levels>(json.take()).unwrap();
    let current = CurrentLevel(levels.0[0].clone());
    commands.insert_resource(levels);
    commands.insert_resource(current);
}

pub fn spawn_map(
    mut commands: Commands,
    current: Res<CurrentLevel>,
    asset_server: Res<AssetServer>,
    mut json_data: ResMut<Assets<JsonFile>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Spawning level");

    // TODO: remove this
    commands.spawn(Camera2dBundle::default());

    let path = &current.0.map;
    let handle: Handle<JsonFile> = asset_server.get_handle(path);
    let json = json_data
        .get_mut(&handle)
        .expect("Failed to get JSON data.");

    let tiled_map_data = serde_json::from_value::<TiledMap>(json.clone()).unwrap();

    // store tileset data and their gids.
    let mut tilesets = vec![];

    for tileset in tiled_map_data.tilesets.iter() {
        match tileset {
            Tileset::External { first_gid, source } => {
                // HACK
                let source = &source[2..];

                let handle: Handle<JsonFile> = asset_server.get_handle(source);
                let json_data = json_data
                    .get_mut(&handle)
                    .expect("Failed to get JSON data.");

                let tileset_data =
                    serde_json::from_value::<TiledTileset>(json_data.clone()).unwrap();

                let properties = PropertyMap::new(&tileset_data);

                tilesets.push((*first_gid as u32, tileset_data, properties));
            }
        }
    }

    let mut gids = HashMap::new();

    for (i, (gid, tileset, _)) in tilesets.iter().enumerate() {
        for j in 0..tileset.tile_count {
            gids.insert((gid + j) as u32, i);
        }
    }

    for (_i, layer) in tiled_map_data.layers.iter().enumerate() {
        match layer {
            TiledLayer::TileLayer(layer) => {
                let width = layer.width;
                let height = layer.height;

                let mut idx = 0;

                for y in 0..height {
                    for x in 0..width {
                        let value = layer.data[idx];
                        idx += 1;

                        if value == 0 {
                            continue;
                        }

                        let x = (x * tiled_map_data.tile_width) as f32;
                        // Since these Tiled maps use a top-down render order the
                        // y coordinates have to be flipped.
                        let y = tiled_map_data.height - 1 - y;
                        let y = (y * tiled_map_data.tile_height) as f32;

                        let tileset_id = gids.get(&value).unwrap();

                        let (gid, tileset, props) = &tilesets[*tileset_id];

                        // HACK
                        let source = &tileset.image[2..];

                        let texture_handle = asset_server.get_handle(source);

                        let rows = tileset.image_width / tileset.tile_width;

                        let id = (value - gid) as f32;
                        let atlas_col = tileset.columns as f32;

                        let src_y = (id / atlas_col).floor() * tileset.tile_height as f32;
                        let src_x = ((id % atlas_col) * tileset.tile_width as f32).ceil();

                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
                            tileset.columns as usize,
                            rows as usize,
                            None,
                            Some(Vec2::new(src_x, src_y)),
                        );

                        let texture_atlas_handle = atlases.add(texture_atlas);

                        commands.spawn((
                            Tile,
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle.clone(),
                                transform: Transform::from_xyz(x, y, 100.),
                                ..default()
                            },
                        ));

                        if let Some(props) = props.0.get(&(id as u32)) {
                            for _prop in props.properties.iter() {}
                        }
                    }
                }
            } // TiledLayer::ObjectLayer => todo!(),
        }
    }
}

fn despawn_map(mut commands: Commands, tiles: Query<Entity, With<Tile>>) {
    for tile in tiles.iter() {
        commands.entity(tile).despawn();
    }
}

fn level_transition(
    mut current_level: ResMut<CurrentLevel>,
    levels: Res<Levels>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let level = &mut current_level.0;

    // if there's another level after this transition to it.
    if let Some(next_id) = &level.next {
        info!("Transitioning to next level: '{}'", next_id);

        let next_level = levels.get(next_id);
        current_level.0 = next_level.clone();

        next_state.set(GameState::Playing);
    }
}
