use bevy::prelude::*;

use crate::images::Images;

pub struct Upkeep(pub i32);

pub struct Revenue(pub i32);

pub struct RevenueTickThreshold(pub i32);

pub struct UpkeepTickThreshold(pub i32);

pub struct HasConstruction(pub Entity);

pub struct Consumer;

pub struct Occupied;

pub struct BuildTimer(pub Timer);

#[derive(Clone)]
pub struct BuildingData {
    pub material: Handle<ColorMaterial>,
    pub consumer: bool,
    pub upkeep: i32,
    pub revenue: i32,
    pub upkeep_tick: i32,
    pub revenue_tick: i32,
}

impl BuildingData {
    pub fn new_house(images: &Images) -> Self {
        BuildingData {
            material: images.house.clone(),
            consumer: true,
            upkeep: 1000,
            revenue: 0,
            upkeep_tick: 7,
            revenue_tick: 7,
        }
    }

    pub fn new_market(images: &Images) -> Self {
        BuildingData {
            material: images.market.clone(),
            consumer: false,
            upkeep: 5000,
            revenue: 2000,
            upkeep_tick: 7,
            revenue_tick: 7,
        }
    }
}
