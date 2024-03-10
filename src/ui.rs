use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState::Peg)
            .add_systems(Update, ui);
    }
}

#[derive(Resource)]
enum UiState {
    Peg,
    Ball,
}

fn ui(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("World");
    });
}
