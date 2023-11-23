use std::f32::consts::PI;
use std::time::Duration;

use crate::components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity};
use crate::{
    EnemyCount, GameTextures, WinSize, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE, BASE_SPEED, TIME_STEP,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32(1.))),
                enemy_fire_system.run_if(enemy_should_fire),
                enemy_movement_system,
            )
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_should_fire() -> bool {
    if thread_rng().gen_bool(1. / 60.) {
        true
    } else {
        false
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>
) {
    let now = time.elapsed_seconds_f64() as f32;

    for mut transform in query.iter_mut() {
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        let max_distance = TIME_STEP * BASE_SPEED;

        let dir: f32 = -1.; // 1 for counter clockwise, -1 clockwise
        let (x_pivot, y_pivot) = (0., 0.);
        let (x_radius, y_radius) = (200., 130.);

        let angle = dir * BASE_SPEED * TIME_STEP * now % 360. / PI;

        let x_distance = x_radius * angle.cos() + x_pivot;
        let y_distance = y_radius * angle.sin() + x_pivot;

        let dx = x_org - x_distance;
        let dy = y_org - y_distance;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. { max_distance / distance } else { 0. };

        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_distance) } else { x.min(x_distance) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_distance) } else { y.min(y_distance) };

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
