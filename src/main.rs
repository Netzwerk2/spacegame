use avian3d::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode};

use camera::CameraPlugin;
use cursor::CursorPlugin;
use player::PlayerPlugin;

mod camera;
mod cursor;
mod player;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins // TODO Clean this
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        CameraPlugin,
        CursorPlugin,
        PlayerPlugin,
    ));

    app.run();
}
