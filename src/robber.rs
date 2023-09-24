use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


/**
 * Robber Plugin
 **/
pub struct RobberPlugin {

}

impl Plugin for RobberPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(InputManagerPlugin::<RobberStates>::default())
       .add_systems(Startup, spawn_robber)
       .add_systems(Update, animate_robber);
  }
}

impl Default for RobberPlugin {
  fn default() -> RobberPlugin {
    RobberPlugin {
    }
  }
}

/**
 * Robber Plugin Ends
 **/

#[derive(Component, Clone, Debug)]
struct AnimationIndices {
  first: usize,
  last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Robber;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum RobberStates {
  Idle,
  Run,
  Jump
}

#[derive(Debug)]
struct RobberStatesAnimation {
  tah: Handle<TextureAtlas>,
  ai: AnimationIndices
}

#[derive(Component, Debug)]
struct HSAList {
  items: Vec<RobberStatesAnimation>
}

fn build_sprite_sheets(
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>) -> HSAList {

  let sheets = [("robber-idle.png", 10, 700.0, 700.0),
                ("robber-run.png",8, 700., 700.),
                ("robber-jump.png", 5, 700.0, 700.0)];

  let mut ret_val = HSAList { items: Vec::new() };

  for tpl in sheets {
    let texture_handle = asset_server.load(tpl.0);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(tpl.2, tpl.3), tpl.1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: tpl.1 - 1 };

    let hsa = RobberStatesAnimation {
      tah: texture_atlas_handle,
      ai: animation_indices
    };

    ret_val.items.push(hsa);
  }
  ret_val
}

fn spawn_robber(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  texture_atlases: ResMut<Assets<TextureAtlas>>) {
  let sprite_sheets = build_sprite_sheets(asset_server, texture_atlases);
  //commands.insert_resource(sprite_sheets.clone());
  commands
    .spawn((
      InputManagerBundle::<RobberStates> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([(KeyCode::Space, RobberStates::Idle)]),
      },
      SpriteSheetBundle {
        texture_atlas: sprite_sheets.items[1].tah.clone(),
        sprite: TextureAtlasSprite::new(sprite_sheets.items[1].ai.first),
        transform: Transform::from_scale(Vec3::splat(0.2)).with_translation(Vec3::splat(-100.)),
        ..default()
      },
      sprite_sheets.items[1].ai.clone(),
      sprite_sheets,
      AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
      Robber,
    ));
}

fn animate_robber(
  time: Res<Time>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut Handle<TextureAtlas>,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &mut AnimationIndices,
    &HSAList
  ), With<Robber>>,
) {

  for (mut hTA, mut timer, mut sprite, mut indices, hsa_list) in &mut query {
    println!("sprite index is {}", sprite.index);
    timer.tick(time.delta());
    if timer.just_finished() {
      sprite.index = if sprite.index == indices.last {
        indices.first
      } else {
        sprite.index + 1
      };
    }
  }
}
