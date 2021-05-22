use bevy::prelude::*;

use crate::{
    bob::{BoardObjectBundle, Coords},
    cursor::Cursor,
    date::{Date, DateTickEvent},
    images::Images,
    map::{TileSize, Tiles},
    Budget,
};

#[derive(Clone)]
pub struct BuildingData {
    material: Handle<ColorMaterial>,
    consumer: bool,
    upkeep: i32,
    revenue: i32,
    upkeep_tick: i32,
    revenue_tick: i32,
}

pub struct Upkeep(i32);

pub struct Revenue(i32);

pub struct RevenueTickThreshold(pub i32);

pub struct UpkeepTickThreshold(pub i32);

pub struct Consumer;

pub struct BuildTimer(pub Timer);

pub struct HasConstruction(pub Entity);

pub struct Occupied;

pub struct SelectedBuilding {
    build_time: f32,
    data: BuildingData,
}

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
            data: BuildingData {
                material: images.house.clone(),
                consumer: true,
                upkeep: 1000,
                revenue: 0,
                upkeep_tick: 7,
                revenue_tick: 7,
            },
        });
    } else if input.just_pressed(KeyCode::Key2) {
        commands.insert_resource(SelectedBuilding {
            build_time: 4.5,
            data: BuildingData {
                material: images.market.clone(),
                consumer: false,
                upkeep: 5000,
                revenue: 2000,
                upkeep_tick: 7,
                revenue_tick: 7,
            },
        });
    }
}

pub fn building(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Handle<ColorMaterial>,
        &mut BuildTimer,
        &BuildingData,
    )>,
) {
    for (entity, mut mat, mut timer, data) in query.iter_mut() {
        timer.0.tick(time.delta());

        if !timer.0.just_finished() {
            continue;
        }

        *mat = data.material.clone();

        commands
            .entity(entity)
            .remove::<BuildTimer>()
            .remove::<BuildingData>();

        if data.consumer {
            commands.entity(entity).insert(Consumer);
        }

        if data.revenue > 0 {
            commands
                .entity(entity)
                .insert(Revenue(data.revenue))
                .insert(RevenueTickThreshold(data.revenue_tick));
        }

        if data.upkeep > 0 {
            commands
                .entity(entity)
                .insert(Upkeep(data.upkeep))
                .insert(UpkeepTickThreshold(data.upkeep_tick));
        }
    }
}

pub fn tick(
    mut budget: ResMut<Budget>,
    date: Res<Date>,
    tiles: Res<Tiles>,
    mut reader: EventReader<DateTickEvent>,
    consumers: Query<&Consumer>,
    revenues: Query<(&Revenue, &RevenueTickThreshold, &Coords)>,
    upkeeps: Query<(&Upkeep, &UpkeepTickThreshold)>,
    constructions: Query<&HasConstruction>,
) {
    // for each date tick event
    for _ in reader.iter() {
        let prev = budget.0;

        // add revenue for each consumer building on a neighbored tile
        for (revenue, threshold, coords) in revenues.iter() {
            if date.0 % threshold.0 != 0 {
                continue;
            }

            for n_cell in coords.get_neighbors().iter() {
                // if neighbored cell has a tile, assign tile, else continue with next loop iteration
                let tile = match tiles.0.get(n_cell) {
                    Some(tile) => tile,
                    None => continue,
                };

                // if neighbored tile has construction, assign it's entity, else continue with next loop iteration
                let building_entity = match constructions.get(*tile) {
                    Ok(has_construction) => has_construction.0,
                    Err(_) => continue,
                };

                // add revenue if building has consumer component
                if consumers.get(building_entity).is_ok() {
                    budget.0 += revenue.0;
                }
            }
        }

        // subtract all upkeeps from budget
        for (upkeep, threshold) in upkeeps.iter() {
            if date.0 % threshold.0 != 0 {
                continue;
            }

            budget.0 -= upkeep.0;
        }

        println!(
            "Old: {}, New: {}, Diff: {}",
            prev,
            budget.0,
            budget.0 - prev
        );
    }
}
