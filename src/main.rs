#![allow(unused)] // silence unused warnings while exploring 

use crate::components::{Player, Velocity, Movable};
use bevy::{prelude::*, window::PrimaryWindow};
use player::PlayerPlugin;

mod components;
mod player;

// region:      --- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const SPRITE_SCALE: f32 = 0.5;
// endregion:   --- Asset Constants

// region:      --- Game Constants
const TIME_STEP: f32 = 1./60.;
const BASE_SPEED: f32 = 500.;
// endregion    --- Game Constants

// region:      --- Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}
// endregion:   --- Resources

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
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, movable_system)
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

    // Add WinSize resource
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // Add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
