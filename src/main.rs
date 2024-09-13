use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use iyes_perf_ui::prelude::*;

#[derive(Resource)]
struct Tickrate {
    timer: Timer,
}

#[derive(Component)]
struct A();

#[derive(Component)]
struct Color2(f32, f32, f32);

#[derive(Component)]
struct Work {
    ticks_remaining: i32,
    total_ticks: i32,
}
impl Work {
    fn new(total_ticks: i32) -> Self {
        Work {
            ticks_remaining: total_ticks,
            total_ticks,
        }
    }
    fn new_with_ticks(total_ticks: i32, ticks_remaining: i32) -> Self {
        Work {
            ticks_remaining,
            total_ticks,
        }
    }
}

#[derive(Component)]
struct Value {
    count: i32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PerfUiCompleteBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                ..Default::default()
            },
            A(),
            Color2(0.0, 1.0, 0.0),
            Work::new(4),
            Value { count: 0 },
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("0", TextStyle { ..default() }),
                transform: Transform::from_xyz(0.0, 50.0, 0.0),
                ..default()
            });
        });

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(150.0, 0.0, 0.0),
                ..Default::default()
            },
            A(),
            Color2(0.0, 0.0, 1.0),
            Work::new(5),
            Value { count: 0 },
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("0", TextStyle { ..default() }),
                transform: Transform::from_xyz(0.0, 50.0, 0.0),
                ..default()
            });
        });
}

fn generator(
    tickrate: Res<Tickrate>,
    mut query: Query<(&mut Sprite, &Color2, &mut Work, &mut Value), With<A>>,
) {
    if tickrate.timer.just_finished() {
        for (mut sprite, color, mut work, mut value) in query.iter_mut() {
            work.ticks_remaining -= 1;
            if work.ticks_remaining == 0 {
                sprite.color = Color::srgb(color.0, color.1, color.2);
                value.count += 1;
                work.ticks_remaining = work.total_ticks;
            } else {
                sprite.color = Color::srgb(1.0, 1.0, 1.0);
            }
        }
    }
}

fn generator_text(query: Query<(&Value, &Children), With<A>>, mut child_query: Query<&mut Text>) {
    for (value, children) in &query {
        for child in children {
            if let Ok(mut text) = child_query.get_mut(*child) {
                text.sections[0].value = value.count.to_string();
            }
        }
    }
}

fn tick(mut tickrate: ResMut<Tickrate>, time: Res<Time>) {
    tickrate.timer.tick(time.delta());
}

fn egui_ui_system(mut contexts: EguiContexts, query: Query<&Work>) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        for work in &query {
            ui.label(work.ticks_remaining.to_string());
        }
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, egui_ui_system)
        .insert_resource(Tickrate {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        })
        .add_systems(Update, (generator, generator_text))
        .add_systems(Update, tick)
        .run();
}
