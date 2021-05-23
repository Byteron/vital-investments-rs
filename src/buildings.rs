use std::fs;

use serde::{Deserialize, Serialize};

use bevy::{prelude::*, utils::HashMap};
#[derive(Deserialize, Serialize, Clone)]
pub struct BuildingData {
    pub image: String,
    pub consumer: bool,
    pub upkeep: i32,
    pub upkeep_tick: i32,
    pub revenue: i32,
    pub revenue_tick: i32,
}

pub struct Buildings {
    map: HashMap<String, BuildingData>,
}

impl Buildings {
    pub fn load(&mut self, key: &str, path: &str) {
        let json = fs::read_to_string(path).unwrap();
        let data: BuildingData = serde_json::from_str(json.as_str()).unwrap();

        self.map.insert(key.to_string(), data);
    }

    pub fn get(&self, key: &str) -> BuildingData {
        self.map
            .get(key)
            .expect(format!("building {} does not exist!", key).as_str())
            .clone()
    }
}

impl FromWorld for Buildings {
    fn from_world(_world: &mut World) -> Self {
        let mut datas = Buildings {
            map: HashMap::default(),
        };

        datas.load("house", "data/buildings/house.json");
        datas.load("market", "data/buildings/market.json");

        datas
    }
}
