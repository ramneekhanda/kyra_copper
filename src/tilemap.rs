use bevy::{prelude::*, asset::{LoadContext, LoadedAsset}, utils::BoxedFuture, reflect::{TypeUuid, TypePath}};
use serde::{Serialize, Deserialize};
use bevy::asset::AssetLoader;
use serde_yaml;
use bevy_rapier2d::prelude::*;

pub struct LevelBuilderPlugin {}

impl Plugin for LevelBuilderPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_tilemap);
    app.add_systems(Update, (load_levels, set_scene));
  }
}

impl Default for LevelBuilderPlugin {
  fn default() -> LevelBuilderPlugin {
    LevelBuilderPlugin {
    }
  }
}

#[derive(Resource, Default)]
pub struct LevelManagerAsset {
  handle: Handle<YamlAsset>,
  level_manager: LevelManager,
  loaded: bool,
  rendered: bool,
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
pub struct YamlAsset(pub String);

#[derive(Deserialize, Debug, Default)]
pub struct LevelManager {
  levels: Vec<Level>
}

#[derive(Deserialize, Debug)]
pub struct Level {
  level: i32,
  floor_tile: String,
  floor_plan: FloorPlan,

  #[serde(skip_deserializing)]
  floor_tile_txt_handle: Handle<Image>,
}

#[derive(Deserialize, Debug)]
pub struct FloorPlan {
  floor_levels: i32,
  level_width: i32,
  level_space: i32,
  elevator: Vec<i32>,
  escalator: Vec<i32>
}

#[derive(Default)]
pub struct YamlLoader;

impl AssetLoader for YamlLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
    Box::pin(async move {
      let data_str = std::str::from_utf8(bytes)?;
      let asset = YamlAsset( data_str.into() );
      load_context.set_default_asset(LoadedAsset::new(asset));
      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["yaml"]
  }
}

fn init_tilemap(
  mut commands: Commands,
  mut lm_asset: ResMut<LevelManagerAsset>,
  asset_server: Res<AssetServer>,
) {
  lm_asset.handle = asset_server.load("floor_plan_1.yaml");
}

fn load_levels(yaml_asset: Res<Assets<YamlAsset>>,
               mut lm_asset: ResMut<LevelManagerAsset>,
               asset_server: Res<AssetServer>,) {

  if lm_asset.loaded == true {
    return;
  }
  let data_str = yaml_asset.get(&lm_asset.handle);
  if data_str.is_none() {
    return;
  }

  lm_asset.level_manager = serde_yaml::from_str(&data_str.unwrap().0).unwrap();
  for lvl in lm_asset.level_manager.levels.iter_mut() {
    lvl.floor_tile_txt_handle = asset_server.load(lvl.floor_tile.clone());
  }
  lm_asset.loaded = true;
  info!("Loaded textures for floor plan {:?}", lm_asset.level_manager);
}

fn set_scene(mut lm_asset: ResMut<LevelManagerAsset>,
             mut commands: Commands,
             window: Query<&Window>,) {
  if !lm_asset.loaded || lm_asset.rendered == true {
    return
  }

  let lm = &lm_asset.level_manager;
  let lvl = &lm.levels[0];
  let ht = window.single().resolution.height();
  let wd = window.single().resolution.width();
  let start_top = (- ht / 2.0) + 16.;
  let start_lft = (- wd / 2.0);
  for floor_no in 0..lvl.floor_plan.floor_levels {
    for tile_no in 0..lvl.floor_plan.level_width {
      let lft = start_lft + 32. * tile_no as f32;
      let top = start_top + floor_no as f32 * lvl.floor_plan.level_space as f32;
      commands.spawn((
        SpriteBundle {
          transform: Transform {
            translation: Vec3::new(lft, top, 0.),
            ..Default::default()
          },
          texture: lvl.floor_tile_txt_handle.clone(),
          ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(16., 16.),
      ));
    }
  }
  lm_asset.rendered = true;
}
