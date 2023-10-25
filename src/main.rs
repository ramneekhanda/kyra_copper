
mod hero;
mod robber;
mod tilemap;

use bevy::{prelude::*, window::{PrimaryWindow, PresentMode}, window::{WindowTheme}, window::WindowMode, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use hero::HeroPlugin;
use robber::RobberPlugin;

fn main() {
  App::new()
    //.insert_resource(ClearColor(Color::rgb(0.2, 0.1, 0.2)))
    .add_plugins((DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Kyra Copper!".into(),
                    resolution: (800., 1024.).into(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: false,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
    .insert_resource(RapierConfiguration {
        gravity: Vect::new(0., -9800.),
        ..Default::default()
    })
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_asset::<tilemap::YamlAsset>()
    .init_asset_loader::<tilemap::YamlLoader>()
    .init_resource::<tilemap::LevelManagerAsset>()
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, setup_camera)
    .add_plugins(tilemap::LevelBuilderPlugin::default())
    .add_plugins((HeroPlugin::default(), RobberPlugin::default()))
    .run();
}

fn setup_camera(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>
) {
  commands.spawn(Camera2dBundle{

    ..Default::default()
  });
}
