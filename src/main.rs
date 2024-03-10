use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use bevy_file_dialog::prelude::*;

mod camera;
mod pegs;
mod ui;

use camera::CameraPlugin;
use pegs::PegPlugin;
use ui::UiPlugin;

pub struct TextFileContents;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(
            FileDialogPlugin::new()
                // allow saving of files marked with TextFileContents
                .with_save_file::<TextFileContents>()
                // allow loading of files marked with TextFileContents
                .with_load_file::<TextFileContents>(),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PegPlugin)
        .add_plugins(UiPlugin)
        .run();
}
