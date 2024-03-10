use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::pegs::{delete_all_objects, Ball, BallSpawner, DeleteObjects, Object, SceneObjects, SpawnObject};
use crate::TextFileContents;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState { started: false })
            .insert_resource(Time::<Virtual>::default())
            .add_systems(Update, ui)
            .add_systems(Update, load_save_file.after(delete_all_objects));
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
    scene_objects: ResMut<SceneObjects>,
    query_balls: Query<Entity, With<Ball>>,
    query_ball_spawners: Query<&Transform, With<BallSpawner>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
    mut commands: Commands,
    mut delete_event_writer: EventWriter<DeleteObjects>,
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
                    spawn_event_writer.send(SpawnObject(Object::Ball(transform.translation.x, transform.translation.y)));
                }
            }
        }
        if ui.button("Save").clicked() {
            commands
                .dialog()
                .save_file::<TextFileContents>(rmp_serde::to_vec(&scene_objects.clone()).unwrap());
        }
        if ui.button("Load").clicked() {
            commands
                .dialog()
                .load_file::<TextFileContents>();
            delete_event_writer.send(DeleteObjects);
        }
    });
}


fn load_save_file(
    mut ev_loaded: EventReader<DialogFileLoaded<TextFileContents>>,
    mut scene_objects: ResMut<SceneObjects>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    for ev in ev_loaded.read() {
        *scene_objects = rmp_serde::from_slice(&ev.contents).unwrap();
        for object in scene_objects.objects.values() {
            spawn_event_writer.send(SpawnObject(object.clone()));
        }
    }
}
