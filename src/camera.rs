use bevy::{app::AppExit, prelude::*};
use bevy::core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, close_window);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Background;

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    )).insert(MainCamera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("white.png"),
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::splat(5000.)),
            ..default()
        },
        ..default()
    }).insert(Background);
}

pub fn close_window(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
}
