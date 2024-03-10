use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;

mod camera;
mod pegs;
mod ui;

use camera::CameraPlugin;
use pegs::PegPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PegPlugin)
        .add_plugins(UiPlugin)
        .run();
}
