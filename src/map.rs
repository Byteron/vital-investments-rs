use bevy::{prelude::*, utils::HashMap};

use crate::{bob::BoardObjectBundle, building::components::Occupied, images::Images};

pub struct GridSize(pub IVec2);

pub struct TileSize(pub IVec2);

pub struct Tiles(pub HashMap<IVec2, Entity>);

pub fn setup(
    mut commands: Commands,
    images: Res<Images>,
    mut tiles: ResMut<Tiles>,
    grid_size: Res<GridSize>,
    tile_size: Res<TileSize>,
) {
    for y in 0..grid_size.0.y {
        for x in 0..grid_size.0.x {
            let cell = IVec2::new(x, y);

            let material: Handle<ColorMaterial>;
            let occupied: bool;

            if x % 4 == 0 || y % 4 == 0 {
                occupied = true;
                material = images.grass.clone();
            } else {
                occupied = false;
                material = images.concrete.clone();
            }

            let entity = commands
                .spawn()
                .insert_bundle(BoardObjectBundle::new(cell, 0, material, tile_size.0))
                .id();

            if occupied {
                commands.entity(entity).insert(Occupied);
            }

            tiles.0.insert(cell, entity);
        }
    }
}
