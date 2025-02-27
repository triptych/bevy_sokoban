use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn create_box_spots(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions_by_color: HashMap<BoxColour, Vec<Position>>,
) {
    for (colour, positions) in positions_by_color {
        let material = materials.add(
            asset_server
                .load(format!("assets/images/box_spot_{}.png", colour))
                .unwrap()
                .into(),
        );

        for position in positions {
            let translation = position_to_translation(map, tile_size, &position, 9.0);

            commands
                .spawn(SpriteComponents {
                    material,
                    translation,
                    ..Default::default()
                })
                .with(position)
                .with(BoxSpot { colour });
        }
    }
}
