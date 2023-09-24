mod hero;
mod robber;

use bevy::{prelude::*, window::PrimaryWindow, window::WindowMode};

use hero::HeroPlugin;
use robber::RobberPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
    .add_systems(Startup, setup_camera)
    .add_plugins((HeroPlugin::default(), RobberPlugin::default()))
    .run();
}

fn setup_camera(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>
) {
  let window = window_query.get_single().unwrap();
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
