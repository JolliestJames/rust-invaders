#![allow(unused)] // silence unused warnings while exploring 

use bevy::{prelude::*, window::PrimaryWindow};

// region:      --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

// endregion:   --- Asset Constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // position:
                resolution: (600., 320.).into(),
                title: "Rust Invaders!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // add 2D camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let Ok(mut primary) = primary_query.get_single_mut() else {
        return;
    };
    let (win_w, win_h) = (primary.width(), primary.height());

    // position window for development
    primary.position = WindowPosition::At(IVec2::new(2100, 0));

    // add player
    let bottom = -win_h / 2.;
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 * SPRITE_SCALE / 2. + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    });
}
