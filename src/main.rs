use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use building::BuildingDatas;
use cursor::Cursor;
use date::{Date, DateTickEvent, DateTimer};
use images::Images;
use map::{GridSize, TileSize, Tiles};

mod bob;
mod building;
mod camera;
mod cursor;
mod date;
mod images;
mod map;

pub struct Budget(pub i32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<DateTickEvent>()
        .init_resource::<Images>()
        .init_resource::<BuildingDatas>()
        .insert_resource(Budget(35000))
        .insert_resource(Date(0))
        .insert_resource(DateTimer(Timer::new(Duration::from_secs_f32(1.0), true)))
        .insert_resource(Cursor(IVec2::ZERO))
        .insert_resource(GridSize(IVec2::splat(21)))
        .insert_resource(TileSize(IVec2::splat(64)))
        .insert_resource(Tiles(HashMap::default()))
        .add_startup_system(setup.system())
        .add_startup_system(date::setup.system())
        .add_startup_system(camera::setup.system())
        .add_startup_system(map::setup.system())
        .add_system(date::tick.system())
        .add_system(cursor::update.system())
        .add_system(camera::movement.system())
        .add_system(building::selection.system())
        .add_system(building::placement.system())
        .add_system(building::building.system())
        .add_system(building::tick.system())
        .add_system(date::update_text.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert_bundle(UiCameraBundle::default());
}
