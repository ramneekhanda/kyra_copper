use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::{RigidBodyForces, AngularInertia}};


/**
 * Hero Plugin
 **/
pub struct HeroPlugin {

}

impl Plugin for HeroPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(InputManagerPlugin::<HeroStates>::default())
      .insert_resource(HSAList {
        items: vec!()
      })
       .add_systems(Startup, (build_sprite_sheets.before(spawn_hero), spawn_hero))
       .add_systems(Update, (animate_hero, move_camera));
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

#[derive(Clone, Debug, Copy)]
struct AnimationIndices {
  first: usize,
  last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Hero;

#[derive(Component, Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum HeroStates {
  Idle = 0,
  Jump = 1,
  RunLeft = 2,
  RunRight = 3
}

#[derive(Clone, Debug)]
struct HeroStatesAnimation {
  tah: Handle<TextureAtlas>,
  ai: AnimationIndices
}

#[derive(Resource, Clone, Debug)]
struct HSAList {
  items: Vec<HeroStatesAnimation>
}

fn build_sprite_sheets(
  mut hsa_list: ResMut<HSAList>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

  let sheets = [("hero-idle.png", 10, 253.0, 389.0),
                ("hero-jump.png", 15, 286.0, 435.0),
                ("hero-run.png", 10, 262.0, 409.0),
                ("hero-run.png", 10, 262.0, 409.0),];

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
    println!("loading resource");
    ret_val.items.push(hsa);
  }
  *hsa_list = ret_val;
}

fn spawn_hero(
  mut commands: Commands,
  sprite_sheet: Res<HSAList>) {
  if sprite_sheet.items.len() == 0 {
    return;
  }
  commands
    .spawn(
      (
        RigidBody::Dynamic,
        ExternalForce {
            force: Vec2::new(0., 0.),
            torque: 0.,
        },
        Velocity { linvel: Vec2::new(0., 0.), angvel: 0. },
        GravityScale(1.),
        AdditionalMassProperties::Mass(100.),
        Collider::capsule_y(20., 170.),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
      )
    )
    .insert((
      InputManagerBundle::<HeroStates> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([(KeyCode::Space, HeroStates::Idle)
                                  , (KeyCode::D, HeroStates::RunRight)
                                  , (KeyCode::A, HeroStates::RunLeft)
                                  , (KeyCode::W, HeroStates::Jump)
        ]),

      },
      HeroStates::Idle,
      SpriteSheetBundle {
        texture_atlas: sprite_sheet.items[0].tah.clone(),
        sprite: TextureAtlasSprite::new(sprite_sheet.items[0].ai.first),
        transform: Transform::from_scale(Vec3::splat(0.2)),
        ..default()
      },
      AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
      Hero,
    ));
}

fn animate_hero(
  time: Res<Time>,
  hsa_list: Res<HSAList>,
  mut query: Query<(
    &mut Handle<TextureAtlas>,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &mut Transform,
    &mut HeroStates,
    &ActionState<HeroStates>
  ), With<Hero>>,
  mut query_rb: Query<(&mut ExternalForce, &mut Velocity)>
) {
  if hsa_list.items.len() == 0 {
    return;
  }
  for (mut tah, mut timer, mut sprite, mut t, mut hero_state, act_state) in &mut query {

    if act_state.just_pressed(HeroStates::Idle) {
      *hero_state = HeroStates::Idle;
    }
    if act_state.just_pressed(HeroStates::RunRight) {
      *hero_state = HeroStates::RunRight;
    }
    if act_state.just_pressed(HeroStates::RunLeft) {
      *hero_state = HeroStates::RunLeft;
    }
    if act_state.just_pressed(HeroStates::Jump) {
      *hero_state = HeroStates::Jump;
    }

    let idx : usize = *hero_state as usize;
    let indices = hsa_list.items[idx].ai;
    *tah = hsa_list.items[idx].tah.clone();
    timer.tick(time.delta());

    if timer.just_finished() {
      sprite.index = if sprite.index >= indices.last {
        indices.first
      } else {
        sprite.index + 1
      };
    }
    if *hero_state == HeroStates::RunRight {
      for (_, mut v) in &mut query_rb {
        v.linvel.x = 500.;
        v.linvel.y = 0.;
      }
    }
    if *hero_state == HeroStates::RunLeft {
      for (_, mut v) in &mut query_rb {
        v.linvel.x = -500.;
        v.linvel.y = 0.;
      }
    }
    if *hero_state == HeroStates::Jump {
      for (mut extf, mut v) in &mut query_rb.iter_mut() {
        v.linvel.y = 300.;
      }
    }
  }

}

fn move_camera(
  mut camera: Query<(&mut Transform, &Camera), (With<Camera2d>, Without<Hero>)>,
  query: Query<(&Transform, &HeroStates), With<Hero>>,
) {
  let mut t = camera.single_mut();
  t.0.translation.x = query.single().0.translation.x;
}
