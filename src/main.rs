use core::fmt;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

/*
#[derive(Component)]
struct Counter(i32);


fn init_counter(mut commands: Commands) {
    commands.spawn(Counter(0));
}


fn increment_counter(mut query: Query<&mut Counter>) {
    for mut counter in &mut query {
        counter.0 += 1
    }
}
*/

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("A".to_string())));
    commands.spawn((Person, Name("B".to_string())));

    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Person placeholder",
        TextStyle {
            ..Default::default()
        },
    ));
}

fn sync_name_to_text(mut query: Query<(&mut Text, &Name), With<Name>>) {
    for (mut text, name) in &mut query {
        text.sections[0].value = name.0.to_string();
    }
}

fn render_person_name(
    name_query: Query<&Name>,
    mut text_query: Query<&mut Text>,
) {
    let mut to_display = "".to_string();
    for name in &name_query {
        let str = format!("Name: {}\n", name.0);
        to_display.push_str(&str);
    }
    for mut text in &mut text_query {
        text.sections[0].value = to_display.clone();
    }
}
/*
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("aaa {} bbb", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "A" {
            name.0 = "AA".to_string();
            break; // We don't need to change any other names.
        }
    }
}
*/
//fn update_text(counter_query: Query<&Counter>, mut text_query: Query<&mut Text>) {
//let counter = counter_query.single();
//let mut text = text_query.single_mut();

//text.sections[0].value = counter.0.to_string()
//}
/*
fn update_person_text(name_query: Query<&Name>, mut text_query: Query<&mut Name, With<Text>>) {
    let name = name_query.single();
    let mut text = text_query.single_mut();

    text.0 = name.0.to_string();
}

fn update_persontext_self(mut nametext_query: Query<(&mut Text, &mut Name), With<Name>>) {
    let (mut text, name) = nametext_query.single_mut();

    text.sections[0].value = name.0.to_string();
}
*/
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, add_people)
        .add_systems(Update, (sync_name_to_text, render_person_name))
        .run();
}
// .add_systems(Startup, init_counter)
// (increment_counter, update_text).chain()
// (update_person_text, update_persontext_self).chain()
