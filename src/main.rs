use bevy::{prelude::*, utils::HashMap};
use cursor::Cursor;
use images::Images;
use map::{GridSize, TileSize, Tiles};

mod bob;
mod building;
mod camera;
mod cursor;
mod images;
mod map;

pub struct Budget(pub i32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Images>()
        .insert_resource(Budget(35000))
        .insert_resource(Cursor(IVec2::ZERO))
        .insert_resource(GridSize(IVec2::splat(21)))
        .insert_resource(TileSize(IVec2::splat(64)))
        .insert_resource(Tiles(HashMap::default()))
        .add_startup_system(camera::setup.system())
        .add_startup_system(map::setup.system())
        .add_system(cursor::update.system())
        .add_system(camera::movement.system())
        .add_system(building::selection.system())
        .add_system(building::placement.system())
        .add_system(building::building.system())
        .run();
}
