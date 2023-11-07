#![allow(unused)] // silence unused warnings while exploring 

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "Rust Invaders!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .run()
}

fn setup_system(mut commands: Commands) {
    // add 2D camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..default()
        },
        ..default()
    });
}