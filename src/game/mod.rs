use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

use cursor::Cursor;
use date::{Date, DateTickEvent, DateTimer};
use map::{GridSize, TileSize, Tiles};

use crate::AppState;

pub mod bob;
pub mod building;
pub mod camera;
pub mod cursor;
pub mod date;
pub mod map;

pub struct Cleanup;

pub struct Budget(pub i32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DateTickEvent>()
            .insert_resource(Budget(35000))
            .insert_resource(Date(0))
            .insert_resource(DateTimer(Timer::new(Duration::from_secs_f32(1.0), true)))
            .insert_resource(Cursor(IVec2::ZERO))
            .insert_resource(GridSize(IVec2::splat(21)))
            .insert_resource(TileSize(IVec2::splat(64)))
            .insert_resource(Tiles(HashMap::default()))
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup.system())
                    .with_system(date::setup.system())
                    .with_system(camera::setup.system())
                    .with_system(map::setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(date::tick.system())
                    .with_system(cursor::update.system())
                    .with_system(camera::movement.system())
                    .with_system(building::selection.system())
                    .with_system(building::placement.system())
                    .with_system(building::building.system())
                    .with_system(building::tick.system())
                    .with_system(date::update_text.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Game)
                    .with_system(crate::despawn_all::<Cleanup>.system()),
            );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(UiCameraBundle::default())
        .insert(Cleanup);
}
