use crate::camera::{Background, MainCamera};
use crate::ui::ui;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_egui::EguiContexts;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub struct PegPlugin;

impl Plugin for PegPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SceneObjects { objects: BTreeMap::new(), object_count: 0 })
            .insert_resource(Octave(3))
            .insert_resource(CurrentDraggedPegId(None))
            .insert_resource(ChordInput { input_active: false, input_notes: Vec::new() })
            .add_event::<SpawnObject>()
            .add_event::<DeleteObjects>()
            .add_systems(FixedUpdate, delete_all_objects)
            .add_systems(FixedUpdate, spawn_object)
            .add_systems(FixedUpdate, spawn_ball.after(ui))
            .add_systems(FixedUpdate, spawn_ball_spawner.after(ui))
            .add_systems(FixedUpdate, cleanup_sounds)
            .add_systems(Startup, setup_sound)
            .add_systems(FixedUpdate, place_peg)
            .add_systems(FixedUpdate, display_events)
            .add_systems(FixedUpdate, drag_peg)
            .add_systems(FixedUpdate, clear_screen);
    
    }
}

#[derive(Resource)]
pub struct Octave(pub u32);

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct SceneObjects {
    pub objects: BTreeMap<u32, Object>,
    object_count: u32
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub enum Object {
    Peg(f32, f32, Vec<u32>),
    Ball(f32, f32),
    BallSpawner(f32, f32),
}

#[derive(Event)]
pub struct SpawnObject(pub Object, pub Option<u32>);

#[derive(Component)]
pub struct Peg;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Copy)]
enum Notes {
    C3,
    CS3,
    D3,
    DS3,
    E3,
    F3,
    FS3,
    G3,
    GS3,
    A3,
    AS3,
    B3,
    C4,
    CS4,
    D4,
    DS4,
    E4,
    F4,
    FS4,
    G4,
    GS4,
    A4,
    AS4,
    B4,
    C5,
}

impl Notes {
    fn to_file_path(&self) -> String {
        match self {
            Notes::C3 => "sounds/c3.ogg",
            Notes::CS3 => "sounds/cs3.ogg",
            Notes::D3 => "sounds/d3.ogg",
            Notes::DS3 => "sounds/ds3.ogg",
            Notes::E3 => "sounds/e3.ogg",
            Notes::F3 => "sounds/f3.ogg",
            Notes::FS3 => "sounds/fs3.ogg",
            Notes::G3 => "sounds/g3.ogg",
            Notes::GS3 => "sounds/gs3.ogg",
            Notes::A3 => "sounds/a3.ogg",
            Notes::AS3 => "sounds/as3.ogg",
            Notes::B3 => "sounds/b3.ogg",
            Notes::C4 => "sounds/c4.ogg",
            Notes::CS4 => "sounds/cs4.ogg",
            Notes::D4 => "sounds/d4.ogg",
            Notes::DS4 => "sounds/ds4.ogg",
            Notes::E4 => "sounds/e4.ogg",
            Notes::F4 => "sounds/f4.ogg",
            Notes::FS4 => "sounds/fs4.ogg",
            Notes::G4 => "sounds/g4.ogg",
            Notes::GS4 => "sounds/gs4.ogg",
            Notes::A4 => "sounds/a4.ogg",
            Notes::AS4 => "sounds/as4.ogg",
            Notes::B4 => "sounds/b4.ogg",
            Notes::C5 => "sounds/c5.ogg",
        }
        .into()
    }
}
fn setup_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/c3.ogg"),
        ..default()
    });
}

fn cleanup_sounds(mut commands: Commands, controller: Query<(Entity, &AudioSink)>) {
    for (id, sink) in controller.iter() {
        if sink.empty() {
            commands.entity(id).despawn();
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    peg_query: Query<(&NotesList, &Sprite)>,
    mut background_query: Query<&mut Sprite, (With<Background>, Without<NotesList>)>,
    asset_server: Res<AssetServer>,
) {
    let Ok(mut background_sprite) = background_query.get_single_mut() else { return };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => {
                let notes;
                let peg_sprite;
                match peg_query.get(*e1) {
                    Ok((n, sprite)) => {
                        notes = n;
                        peg_sprite = sprite;
                    }
                    Err(_) => match peg_query.get(*e2) {
                        Ok((n, sprite)) => {
                            notes = n;
                            peg_sprite = sprite;
                        }
                        Err(_) => return,
                    },
                }
                for note in &notes.0 {
                    commands.spawn(AudioBundle {
                        source: asset_server.load(note.to_file_path()),
                        ..default()
                    });
                }
                
                // use peg_sprite
                background_sprite.color = Color::rgb_from_array(peg_sprite.color.rgb_to_vec3() / 3.5);
                // peg_sprite.color = Color::rgb_from_array((-1. * peg_sprite.color.rgb_to_vec3()).exp());
            }
            _ => {}
        }
    }
    background_sprite.color = Color::rgb_from_array(background_sprite.color.rgb_to_vec3() / 1.1);
}

#[derive(Component)]
pub struct NotesList(Vec<Notes>);

#[derive(Component)]
pub struct BallSpawner;

#[derive(Component)]
struct ObjectId(u32);

fn gaussian_sample(x: f32, mean: f32) -> f32 {
    4. * (-8. * ((x - mean) / 2.).powf(2.)).exp()
}

fn gaussian_sample_triple(mean: f32) -> (f32, f32, f32) {
    (gaussian_sample(0., mean), gaussian_sample(1., mean), gaussian_sample(2., mean))
}

fn spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_objects: ResMut<SceneObjects>,
    mut spawn_events: EventReader<SpawnObject>,
) {
    for ev in spawn_events.read() {
        match ev.0 {
            Object::Peg(x, y, ref notes) => {
                let noteslist = NotesList(notes.iter().map(|&i| convert_index_to_note(i)).collect());
                let (r, g, b) = gaussian_sample_triple(notes[0] as f32 / 24.);
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("peg.png"),
                        sprite: Sprite {
                            color: Color::rgb(r, g, b),
                            custom_size: Some(Vec2::splat(100.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x, y, 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Peg)
                    .insert(ObjectId(ev.1.unwrap_or(scene_objects.object_count)))
                    .insert(RigidBody::Fixed)
                    .insert(Collider::ball(45.))
                    .insert(noteslist)
                    .insert(Restitution {
                        coefficient: 0.7,
                        combine_rule: CoefficientCombineRule::Max,
                    });
                if ev.1.is_none() {
                    let object_count = scene_objects.object_count;
                    scene_objects.objects.insert(object_count, ev.0.clone());
                    scene_objects.object_count += 1;
                }
            }
            Object::Ball(x, y) => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("peg.png"),
                        sprite: Sprite {
                            color: Color::rgb(7.5, 0.0, 7.5),
                            // color: Color::WHITE,
                            custom_size: Some(Vec2::splat(100.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x, y, 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Ball)
                    .insert(GravityScale(2.0))
                    .insert(RigidBody::Dynamic)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Collider::ball(45.));
            }
            Object::BallSpawner(x, y) => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("peg.png"),
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.0, 10.0),
                            custom_size: Some(Vec2::splat(100.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x, y, 1.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(BallSpawner)
                    .insert(ObjectId(ev.1.unwrap_or(scene_objects.object_count)));
                if ev.1.is_none() {
                    let object_count = scene_objects.object_count;
                    scene_objects.objects.insert(object_count, ev.0.clone());
                    scene_objects.object_count += 1;
                }
            }
        }
    }
}

fn spawn_ball(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Right) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_event_writer.send(SpawnObject(Object::Ball(position.x, position.y), None));
        }
    }
}

fn spawn_ball_spawner(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Middle) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_event_writer.send(SpawnObject(Object::BallSpawner(position.x, position.y), None));
        }
    }
}

#[derive(Event)]
pub struct DeleteObjects;

pub fn delete_all_objects(
    mut delete_events: EventReader<DeleteObjects>,
    query_all_objects: Query<Entity, Or<(With<Peg>, With<Ball>, With<BallSpawner>)>>,
    mut scene_objects: ResMut<SceneObjects>,
    mut commands: Commands,
) {
    for _ in delete_events.read() {
        for e in query_all_objects.iter() {
            commands.entity(e).despawn();
        }
        scene_objects.objects.clear();
        scene_objects.object_count = 0;
    }
}

fn clear_screen(
    input: Res<ButtonInput<KeyCode>>,
    mut delete_event_writer: EventWriter<DeleteObjects>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        delete_event_writer.send(DeleteObjects);
    }
}

#[derive(Resource)]
struct ChordInput {
    input_active: bool,
    input_notes: Vec<u32>,
}

fn place_peg(
    input: Res<ButtonInput<KeyCode>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut octave: ResMut<Octave>,
    mut chord_input: ResMut<ChordInput>,
) {
    if chord_input.input_active {
        if input.just_pressed(KeyCode::Enter) {
            chord_input.input_active = false;
            let (camera, camera_transform) = primary_camera.single();
            if chord_input.input_notes.len() > 0 {
                if let Some(position) = primary_window
                    .single()
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    spawn_event_writer.send(SpawnObject(Object::Peg(position.x, position.y, chord_input.input_notes.clone()), None));
                }
            }
            chord_input.input_notes = Vec::new();
            return;
        }
        if input.just_pressed(KeyCode::Digit1) {
            octave.0 = 3;
        }
        if input.just_pressed(KeyCode::Digit2) {
            octave.0 = 4;
        }
        let mut index = if octave.0 == 3 {
            0
        } else {
            12
        };
        let mut shouldspawn = false;
        
        if input.just_pressed(KeyCode::KeyC) {
            shouldspawn = true;
            index += 0;
        }
        if input.just_pressed(KeyCode::KeyD) {
            shouldspawn = true;

            index += 2;
        }
        if input.just_pressed(KeyCode::KeyE) {
            shouldspawn = true;
            index += 4;
        }
        if input.just_pressed(KeyCode::KeyF) {
            shouldspawn = true;
            index += 5;
        }
        if input.just_pressed(KeyCode::KeyG) {
            shouldspawn = true;
            index += 7;
        }
        if input.just_pressed(KeyCode::KeyA) {
            shouldspawn = true;
            index += 9;
        } 
        if input.just_pressed(KeyCode::KeyB) {
            shouldspawn = true;
            index += 11;
        }
        
        
        if input.pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight) {
            if index != 24 {
                index += 1;
            }
        }

        if input.pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight) {
            if index != 0 {
                index -= 1;
            }
        }
        if shouldspawn {
            chord_input.input_notes.push(index);
        }
    } else {
        if input.just_pressed(KeyCode::Enter) {
            chord_input.input_active = true;
        } else {
            if input.just_pressed(KeyCode::Digit1) {
                octave.0 = 3;
            }
            if input.just_pressed(KeyCode::Digit2) {
                octave.0 = 4;
            }
            let mut index = if octave.0 == 3 {
                0
            } else {
                12
            };
            let mut shouldspawn = false;
            
            if input.just_pressed(KeyCode::KeyC) {
                shouldspawn = true;
                index += 0;
            }
            if input.just_pressed(KeyCode::KeyD) {
                shouldspawn = true;

                index += 2;
            }
            if input.just_pressed(KeyCode::KeyE) {
                shouldspawn = true;
                index += 4;
            }
            if input.just_pressed(KeyCode::KeyF) {
                shouldspawn = true;
                index += 5;
            }
            if input.just_pressed(KeyCode::KeyG) {
                shouldspawn = true;
                index += 7;
            }
            if input.just_pressed(KeyCode::KeyA) {
                shouldspawn = true;
                index += 9;
            } 
            if input.just_pressed(KeyCode::KeyB) {
                shouldspawn = true;
                index += 11;
            }
            
            
            if input.pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight) {
                if index != 24 {
                    index += 1;
                }
            }

            if input.pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight) {
                if index != 0 {
                    index -= 1;
                }
            }
            if shouldspawn {
                // chord_input.input_notes.push(index);
                let (camera, camera_transform) = primary_camera.single();
                if let Some(position) = primary_window
                    .single()
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    spawn_event_writer.send(SpawnObject(Object::Peg(position.x, position.y, vec![index]), None));
                }
            }
        }
    }
    
}

fn convert_index_to_note(i: u32) -> Notes {
    match i {
        0 => Notes::C3,
        1 => Notes::CS3,
        2 => Notes::D3,
        3 => Notes::DS3,
        4 => Notes::E3,
        5 => Notes::F3,
        6 => Notes::FS3,
        7 => Notes::G3,
        8 => Notes::GS3,
        9 => Notes::A3,
        10 => Notes::AS3,
        11 => Notes::B3,
        12 => Notes::C4,
        13 => Notes::CS4,
        14 => Notes::D4,
        15 => Notes::DS4,
        16 => Notes::E4,
        17 => Notes::F4,
        18 => Notes::FS4,
        19 => Notes::G4,
        20 => Notes::GS4,
        21 => Notes::A4,
        22 => Notes::AS4,
        23 => Notes::B4,
        24 => Notes::C5,
        _ => Notes::C3,
    }
}

#[derive(Resource)]
struct CurrentDraggedPegId(Option<u32>);

fn drag_peg(
    input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut pegs: Query<(&mut Transform, &ObjectId, Entity), (With<Peg>, Without<BallSpawner>)>,
    mut ball_spawners: Query<(&mut Transform, &ObjectId, Entity), With<BallSpawner>>,
    mut scene_objects: ResMut<SceneObjects>,
    mut current_dragged_peg_id: ResMut<CurrentDraggedPegId>,
    mut commands: Commands,
) {
    let peg_radius = 18.;
    let (camera, camera_transform) = primary_camera.single();

    if keyboard_input.just_pressed(KeyCode::KeyX) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            for (transform, ObjectId(id), entity_id) in pegs.iter() {
                if transform.translation.truncate().distance(position) <= peg_radius {
                    commands.entity(entity_id).despawn();
                    scene_objects.objects.remove(id);
                    return;
                }
            }
            for (transform, ObjectId(id), entity_id) in ball_spawners.iter() {
                if transform.translation.truncate().distance(position) <= peg_radius {
                    commands.entity(entity_id).despawn();
                    scene_objects.objects.remove(id);
                    return;
                }
            }
        }
    } else if input.just_pressed(MouseButton::Left) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            for (transform, ObjectId(id), _) in pegs.iter() {
                if transform.translation.truncate().distance(position) <= peg_radius {
                    current_dragged_peg_id.0 = Some(*id);
                    return;
                }
            }
            for (transform, ObjectId(id), _) in ball_spawners.iter() {
                if transform.translation.truncate().distance(position) <= peg_radius {
                    current_dragged_peg_id.0 = Some(*id);
                    return;
                }
            }
        }
    } else if input.pressed(MouseButton::Left) {
        match current_dragged_peg_id.0 {
            None => {},
            Some(ref id) => {
                if let Some(position) = primary_window
                    .single()
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let object = scene_objects.objects.get_mut(id).unwrap();
                    let x;
                    let y;
                    match object {
                        Object::Peg(ref mut x_, ref mut y_, _) => {
                            x = x_;
                            y = y_;
                        }
                        Object::BallSpawner(ref mut x_, ref mut y_) => {
                            x = x_;
                            y = y_;
                        }
                        _ => panic!("Expected to be dragging peg; was not dragging peg!"),
                    }
                    *x = position.x;
                    *y = position.y;
                    for (mut transform, ObjectId(obj_id), _) in pegs.iter_mut() {
                        if obj_id == id {
                            transform.translation.x = position.x;
                            transform.translation.y = position.y;
                        }
                    }
                    for (mut transform, ObjectId(obj_id), _) in ball_spawners.iter_mut() {
                        if obj_id == id {
                            transform.translation.x = position.x;
                            transform.translation.y = position.y;
                        }
                    }
                }
            }
        }
    } else {
        current_dragged_peg_id.0 = None;
    }
}
