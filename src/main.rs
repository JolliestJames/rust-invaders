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
        .run()
}
