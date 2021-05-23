use bevy::prelude::*;
use buildings::Buildings;
use game::GamePlugin;
use images::Images;

mod buildings;
mod game;
mod images;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .init_resource::<Images>()
        .init_resource::<Buildings>()
        .add_state(AppState::Game)
        .run();
}

pub fn despawn_all<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.despawn_recursive(e);
    }
}
