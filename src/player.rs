use crate::components::{Player, Velocity};
use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP, BASE_SPEED};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system)
        .add_systems(Update, player_movement_system);
    }
}

fn player_spawn_system (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    let bottom = -win_size.h / 2.;
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 * SPRITE_SCALE / 2. + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Velocity {x: 1., y: 0. });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
