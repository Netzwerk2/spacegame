use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, thrust);
    }
}

#[derive(Component)]
pub(crate) struct PlayerMarker;

#[derive(Bundle)]
struct PlayerBundle {
    body: RigidBody,
    mass: Mass,
    collider: Collider,
    locked_axes: LockedAxes,
    scene: SceneBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PlayerBundle {
            body: RigidBody::Dynamic,
            mass: Mass(1.0),
            collider: Collider::sphere(0.5),
            locked_axes: LockedAxes::new()
                .lock_translation_y()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            scene: SceneBundle {
                scene: asset_server.load("debug_ship.glb#Scene0"),
                ..default()
            },
        },
        PlayerMarker,
    ));
}

fn thrust(mut query: Query<&mut LinearVelocity, With<RigidBody>>) {
    for mut velocity in query.iter_mut() {
        **velocity = -12.0 * Vec3::Z;
    }
}
