use bevy::{
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use core::fmt;
use iyes_perf_ui::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Person;

#[derive(Component, PartialEq, Eq)]
struct Name(String);

#[derive(Component, PartialEq, Eq, Clone)]
struct Position(i32, i32);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct NeighbourText;

struct NeighbourData {
    entity: Entity,
    name: String,
    position: Position,
}

impl fmt::Display for NeighbourData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Neighbour {}: coordinates: X:{}, Y:{}. Entity: {}",
            self.name, self.position.0, self.position.1, self.entity
        )
    }
}

#[derive(Component)]
struct Neighbours {
    neighbours: Vec<NeighbourData>,
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Player,
        Person,
        Name("Player".to_string()),
        Position(0, 0),
        Neighbours { neighbours: vec![] },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Person,
        Name("A".to_string()),
        Position(0, 0),
        Neighbours { neighbours: vec![] },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.5),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Person,
        Name("B".to_string()),
        Position(1, 1),
        Neighbours { neighbours: vec![] },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Person,
        Name("C".to_string()),
        Position(2, 2),
        Neighbours { neighbours: vec![] },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "Person placeholder",
            TextStyle {
                ..Default::default()
            },
        ),
        NeighbourText,
    ));
}

fn player_movement(
    mut query: Query<&mut Position, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let mut position = query.single_mut();
        position.1 += 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        let mut position = query.single_mut();
        position.1 -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        let mut position = query.single_mut();
        position.0 += 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        let mut position = query.single_mut();
        position.0 -= 1;
    }
}

fn random_movement(
    mut query: Query<&mut Position, Without<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut position in &mut query {
            position.0 = rand::thread_rng().gen_range(-5..5);
            position.1 = rand::thread_rng().gen_range(-5..5);
        }
    }
}

fn sync_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = (position.0 as f32) * 50.0;
        transform.translation.y = (position.1 as f32) * 50.0;
    }
}

fn greet_neighbours(
    mut person_query: Query<(Entity, &Name, &Position, &mut Neighbours), With<Person>>,
) {
    let people_data: Vec<(Entity, String, Position)> = person_query
        .iter()
        .map(|(entity, name, position, _)| (entity, name.0.clone(), position.clone()))
        .collect();

    for (_, person_name, person_position, mut neighbours) in person_query.iter_mut() {
        let mut new_neighbours: Vec<NeighbourData> = Vec::new();

        for (person2_entity, person2_name, person2_position) in &people_data {
            if person_name.0 != person2_name.to_string() && person_position != person2_position {
                let x_distance = person_position.0 - person2_position.0;
                let y_distance = person_position.1 - person2_position.1;
                let x_distance_abs = x_distance.abs();
                let y_distance_abs = y_distance.abs();

                if x_distance_abs <= 1 && y_distance_abs <= 1 {
                    new_neighbours.push(NeighbourData {
                        name: person2_name.to_string(),
                        position: person2_position.clone(),
                        entity: *person2_entity,
                    });
                }
            }
        }
        neighbours.neighbours = new_neighbours;
    }
}

fn sync_name_to_text(mut query: Query<(&mut Text, &Name), (With<Name>)>) {
    for (mut text, name) in &mut query {
        text.sections[0].value = name.0.to_string();
    }
}

fn render_person_name(
    name_query: Query<(&Name, &Neighbours)>,
    mut text_query: Query<&mut Text, With<NeighbourText>>,
) {
    let mut to_display = "".to_string();
    for (name, neighbours) in &name_query {
        let str = format!(
            "Name: {}. Neighbour count: {}. Neighbour data: {}\n",
            name.0,
            neighbours.neighbours.len(),
            neighbours
                .neighbours
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" - ")
        );
        to_display.push_str(&str);
    }
    for mut text in &mut text_query {
        text.sections[0].value = to_display.clone();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}

fn main() {
    App::new()
        /*.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..Default::default()
            }),
            ..Default::default()
        }))*/
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(bevy::render::settings::Backends::METAL),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_people)
        .add_systems(Update, (sync_name_to_text, render_person_name))
        .add_systems(Update, greet_neighbours)
        .add_systems(Update, (sync_position, random_movement, player_movement))
        .run();
}
