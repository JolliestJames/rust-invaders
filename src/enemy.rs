use crate::{GameTextures, SPRITE_SCALE};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, enemy_spawn_system);
    }

}

fn enemy_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.spawn(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    });
}
