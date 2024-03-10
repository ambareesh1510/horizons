use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::pegs::{Ball, BallSpawner};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState { started: false })
            .insert_resource(Time::<Virtual>::default())
            .add_systems(Update, ui);
    }
}

#[derive(Resource)]
pub struct UiState {
    started: bool,
}

pub fn ui(
    mut contexts: EguiContexts,
    mut time: ResMut<Time<Virtual>>,
    mut ui_state: ResMut<UiState>,
    query_balls: Query<Entity, With<Ball>>,
    query_ball_spawners: Query<&Transform, With<BallSpawner>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    egui::SidePanel::left("").show(contexts.ctx_mut(), |ui| {
        ui.label("Settings");
        if ui.button("Pause").clicked() {
            time.pause();
        }
        if ui.button("Play").clicked() {
            time.unpause();
        }
        if ui_state.started {
            if ui.button("Reset").clicked() {
                ui_state.started = false;
                for e in query_balls.iter() {
                    commands.entity(e).despawn();
                }
            }
        } else {
            if ui.button("Start").clicked() {
                ui_state.started = true;
                for transform in query_ball_spawners.iter() {
                    commands
                        .spawn(SpriteBundle {
                            texture: asset_server.load("peg.png"),
                            transform: Transform {
                                translation: Vec3::new(transform.translation.x, transform.translation.y, 0.),
                                scale: Vec3::new(0.3, 0.3, 1.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Ball)
                        .insert(GravityScale(2.0))
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::ball(25.));
                }
            }
        }
    });
}
