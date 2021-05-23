use bevy::prelude::*;

use crate::game::{camera::MainCamera, map::TileSize};

#[derive(Debug)]
pub struct Cursor(pub IVec2);

pub fn update(
    mut cursor: ResMut<Cursor>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    query: Query<&Transform, With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        let position = pos - size / 2.0;

        let camera_transform = query.single().unwrap();

        let world_matrix = camera_transform.compute_matrix() * position.extend(0.0).extend(1.0);
        let world_position = Vec2::new(world_matrix.x, world_matrix.y) + tile_size.0.as_f32() / 2.0;
        let cell = world_position.as_i32() / tile_size.0;

        cursor.0 = cell;
    }
}
