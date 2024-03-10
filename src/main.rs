use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod pegs;

use camera::CameraPlugin;
use pegs::PegPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PegPlugin)
        // .add_systems(Startup, setup_camera)
        .run();
}
