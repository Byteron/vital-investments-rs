use bevy::prelude::*;

use crate::map::{GridSize, TileSize};

pub struct MainCamera;

pub fn setup(mut commands: Commands, grid_size: Res<GridSize>, tile_size: Res<TileSize>) {
    let center = (grid_size.0.as_f32() * tile_size.0.as_f32()) / 2.0;

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(center.x, center.y, 1000.0 - 0.1))
        .insert(MainCamera);
}

pub fn movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in query.iter_mut() {
        let direction = get_input_direction(&input);
        transform.translation += direction * time.delta_seconds() * 250.0;
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    return direction.normalize_or_zero();
}
