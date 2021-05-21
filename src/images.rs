use bevy::prelude::*;

pub struct Images {
    pub grass: Handle<ColorMaterial>,
    pub concrete: Handle<ColorMaterial>,
    pub house: Handle<ColorMaterial>,
    pub market: Handle<ColorMaterial>,
    pub site: Handle<ColorMaterial>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = cell.get_resource::<AssetServer>().unwrap();

        Images {
            grass: materials.add(assets.load("grass.png").into()),
            concrete: materials.add(assets.load("concrete.png").into()),
            house: materials.add(assets.load("house.png").into()),
            market: materials.add(assets.load("market.png").into()),
            site: materials.add(assets.load("site.png").into()),
        }
    }
}
