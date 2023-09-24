use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


/**
 * Hero Plugin
 **/
pub struct HeroPlugin {

}

impl Plugin for HeroPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(InputManagerPlugin::<HeroStates>::default())
       .add_systems(Startup, spawn_hero)
       .add_systems(Update, animate_hero);
  }
}

impl Default for HeroPlugin {
  fn default() -> HeroPlugin {
    HeroPlugin {
    }
  }
}

/**
 * Hero Plugin Ends
 **/

#[derive(Component, Clone, Debug)]
struct AnimationIndices {
  first: usize,
  last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Hero;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum HeroStates {
  Idle,
  Run,
  Jump
}

#[derive(Debug)]
struct HeroStatesAnimation {
  tah: Handle<TextureAtlas>,
  ai: AnimationIndices
}

#[derive(Component, Debug)]
struct HSAList {
  items: Vec<HeroStatesAnimation>
}

fn build_sprite_sheets(
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>) -> HSAList {

  let sheets = [("hero-idle.png", 10, 253.0, 389.0),
                ("hero-run.png", 10, 262.0, 409.0),
                ("hero-jump.png", 15, 286.0, 435.0)];

  let mut ret_val = HSAList { items: Vec::new() };

  for tpl in sheets {
    let texture_handle = asset_server.load(tpl.0);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(tpl.2, tpl.3), tpl.1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: tpl.1 - 1 };

    let hsa = HeroStatesAnimation {
      tah: texture_atlas_handle,
      ai: animation_indices
    };

    ret_val.items.push(hsa);
  }
  ret_val
}

fn spawn_hero(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  texture_atlases: ResMut<Assets<TextureAtlas>>) {
  let sprite_sheets = build_sprite_sheets(asset_server, texture_atlases);
  //commands.insert_resource(sprite_sheets.clone());
  commands
    .spawn((
      InputManagerBundle::<HeroStates> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([(KeyCode::Space, HeroStates::Idle)]),
      },
      SpriteSheetBundle {
        texture_atlas: sprite_sheets.items[0].tah.clone(),
        sprite: TextureAtlasSprite::new(sprite_sheets.items[0].ai.first),
        transform: Transform::from_scale(Vec3::splat(0.3)),
        ..default()
      },
      sprite_sheets.items[0].ai.clone(),
      sprite_sheets,
      AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
      Hero,
    ));
}

fn animate_hero(
  time: Res<Time>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut Handle<TextureAtlas>,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &mut AnimationIndices,
    &HSAList
  ), With<Hero>>,
) {

  for (mut hTA, mut timer, mut sprite, mut indices, hsa_list) in &mut query {
    println!("sprite index is {}", sprite.index);
    hTA.set(Box::new(hsa_list.items[2].tah.clone()));
    hsa_list.items[2].ai.clone_into(&mut indices);
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
