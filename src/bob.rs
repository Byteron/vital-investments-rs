use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub IVec2);
pub struct Layer(pub i32);

#[derive(Bundle)]
pub struct BoardObjectBundle {
    coords: Coords,
    layer: Layer,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl BoardObjectBundle {
    pub fn new(cell: IVec2, layer: i32, material: Handle<ColorMaterial>, tile_size: IVec2) -> Self {
        let coords = Coords(cell);

        BoardObjectBundle {
            coords,
            layer: Layer(layer),
            sprite_bundle: SpriteBundle {
                material,
                sprite: Sprite::new(tile_size.as_f32()),
                transform: Transform::from_translation((coords.0 * tile_size).extend(layer).as_f32()),
                ..Default::default()
            },
        }
    }
}