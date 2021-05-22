use bevy::{prelude::*, utils::HashMap};

pub struct Images {
    map: HashMap<String, Handle<ColorMaterial>>,
}

impl Images {
    pub fn insert(&mut self, key: &str, handle: Handle<ColorMaterial>) {
        self.map.insert(key.to_string(), handle);
    }

    pub fn get(&self, key: &str) -> Handle<ColorMaterial> {
        self.map
            .get(&key.to_string())
            .expect(format!("image {} does not exist!", key).as_str())
            .clone()
    }
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = cell.get_resource::<AssetServer>().unwrap();

        let mut images = Images {
            map: HashMap::default(),
        };

        images.insert("grass", materials.add(assets.load("grass.png").into()));

        images.insert(
            "concrete",
            materials.add(assets.load("concrete.png").into()),
        );

        images.insert("house", materials.add(assets.load("house.png").into()));
        images.insert("market", materials.add(assets.load("market.png").into()));
        images.insert("site", materials.add(assets.load("site.png").into()));

        images
    }
}
