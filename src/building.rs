use bevy::prelude::*;

use crate::{
    bob::BoardObjectBundle,
    cursor::Cursor,
    images::Images,
    map::{TileSize, Tiles},
};

#[derive(Clone)]
pub struct BuildData {
    pub material: Handle<ColorMaterial>,
}

pub struct SelectedBuilding {
    build_time: f32,
    data: BuildData,
}

pub struct BuildFinishedEvent {
    entity: Entity,
    data: BuildData,
}

pub struct BuildTimer(pub Timer);

pub struct HasConstruction(pub Entity);

pub struct Occupied;

pub fn placement(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    images: Res<Images>,
    selected_building: Option<Res<SelectedBuilding>>,
    tile_size: Res<TileSize>,
    tiles: Res<Tiles>,
    cursor: Res<Cursor>,
    query: Query<&Occupied>,
) {
    if selected_building.is_none() {
        return;
    }
    let selected_building = selected_building.unwrap();
    
    if input.just_pressed(MouseButton::Left) {
        let cell = cursor.0;

        let tile = match tiles.0.get(&cell) {
            Some(tile) => tile,
            None => return,
        };

        if query.get(*tile).is_ok() {
            return;
        }

        let construction = commands
            .spawn()
            .insert_bundle(BoardObjectBundle::new(
                cell,
                1,
                images.site.clone(),
                tile_size.0,
            ))
            .insert(BuildTimer(Timer::from_seconds(
                selected_building.build_time,
                false,
            )))
            .insert(selected_building.data.clone())
            .id();

        commands
            .entity(*tile)
            .insert(HasConstruction(construction))
            .insert(Occupied);
    }
}

pub fn selection(mut commands: Commands, input: Res<Input<KeyCode>>, images: Res<Images>) {
    if input.just_pressed(KeyCode::Key0) {
        commands.remove_resource::<SelectedBuilding>();
    } else if input.just_pressed(KeyCode::Key1) {
        commands.insert_resource(SelectedBuilding {
            build_time: 1.5,
            data: BuildData {
                material: images.house.clone(),
            },
        });
    } else if input.just_pressed(KeyCode::Key2) {
        commands.insert_resource(SelectedBuilding {
            build_time: 4.5,
            data: BuildData {
                material: images.market.clone(),
            },
        });
    }
}

pub fn building(
    time: Res<Time>,
    mut writer: EventWriter<BuildFinishedEvent>,
    mut query: Query<(Entity, &mut BuildTimer, &BuildData)>,
) {
    for (entity, mut timer, data) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            writer.send(BuildFinishedEvent {
                entity,
                data: data.clone(),
            });
        }
    }
}

pub fn construct(
    mut commands: Commands,
    mut reader: EventReader<BuildFinishedEvent>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<ColorMaterial>, &BuildData)>,
) {
    for event in reader.iter() {
        let entity = event.entity;

        match query.get_mut(entity) {
            Ok((mut mat, data)) => {
                *mat = data.material.clone();
        
                commands.entity(entity).remove::<BuildTimer>();
                commands.entity(entity).remove::<BuildData>();
            }
            Err(e) => { println!("{}", e); } // should never happen
        }
    }
}


