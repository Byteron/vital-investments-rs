use bevy::prelude::*;

use serde::{Deserialize, Serialize};

pub struct Upkeep(pub i32);

pub struct Revenue(pub i32);

pub struct RevenueTickThreshold(pub i32);

pub struct UpkeepTickThreshold(pub i32);

pub struct HasConstruction(pub Entity);

pub struct Consumer;

pub struct Occupied;

pub struct BuildTimer(pub Timer);

#[derive(Deserialize, Serialize, Clone)]
pub struct BuildingData {
    pub image: String,
    pub consumer: bool,
    pub upkeep: i32,
    pub upkeep_tick: i32,
    pub revenue: i32,
    pub revenue_tick: i32,
}
