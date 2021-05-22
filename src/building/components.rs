use bevy::prelude::*;

pub struct Upkeep(pub i32);

pub struct Revenue(pub i32);

pub struct RevenueTickThreshold(pub i32);

pub struct UpkeepTickThreshold(pub i32);

pub struct Consumer;

pub struct BuildTimer(pub Timer);

pub struct HasConstruction(pub Entity);

pub struct Occupied;
