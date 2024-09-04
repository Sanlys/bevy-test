use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
#[derive(PartialEq, Eq)]
struct Name(String);

#[derive(Component)]
#[derive(PartialEq, Eq)]
struct Position(i32, i32);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("A".to_string()), Position(0, 0)));
    commands.spawn((Person, Name("B".to_string()), Position(1, 1)));
    commands.spawn((Person, Name("C".to_string()), Position(2, 2)));

    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Person placeholder",
        TextStyle {
            ..Default::default()
        },
    ));
}

fn greet_neighbours(person_query: Query<(&Name, &Position), With<Person>>) {
    for person in &person_query {
        for person2 in &person_query {
            if person.0 != person2.0 && person.1 != person2.1 {
                let x_distance = person.1.0 - person2.1.0;
                let y_distance = person.1.1 - person2.1.1;
                let x_distance_abs = x_distance.abs();
                let y_distance_abs = y_distance.abs();
                if x_distance_abs <= 1 || y_distance_abs <= 1 {
                    println!("{} greets {}. X-distance: {}, Y-distance: {}", person.0.0, person2.0.0, x_distance_abs, y_distance_abs);
                }
            }
        }
    }
}

fn sync_name_to_text(mut query: Query<(&mut Text, &Name), With<Name>>) {
    for (mut text, name) in &mut query {
        text.sections[0].value = name.0.to_string();
    }
}

fn render_person_name(name_query: Query<&Name>, mut text_query: Query<&mut Text>) {
    let mut to_display = "".to_string();
    for name in &name_query {
        let str = format!("Name: {}\n", name.0);
        to_display.push_str(&str);
    }
    for mut text in &mut text_query {
        text.sections[0].value = to_display.clone();
    }
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
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (sync_name_to_text, render_person_name))
        .add_systems(Update, greet_neighbours)
        .run();
}
