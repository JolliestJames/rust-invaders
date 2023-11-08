use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system);
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
    });
}
