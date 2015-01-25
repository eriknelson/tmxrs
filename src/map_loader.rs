use std::collections::HashMap;
use std::default::Default;

use rsfml::system::Vector2f;
use rsfml::graphics::{
  IntRect,
  FloatRect
};

use super::MapOrientation;

// TODO: Fill out map loader, blocked on MapLayer type
pub struct MapLoader<'a> {
  // Tile count
  width: u16,
  height: u16,
  //
  // Tile sizes
  tile_width: u16,
  tile_height: u16,

  orientation: MapOrientation,

  // Width / height isometric tiles
  tile_ratio: f32,

  properties: HashMap<&'a str, &'a str>,

  // Bounding area of tiles visible on screen
  bounds: FloatRect,
}

// Holds texture coords and tileset id of a tile
struct TileInfo {
  coords: [Vector2f; 4],
  size: Vector2f,
  tile_set_id: u16,

}

impl Default for TileInfo {
  fn default() -> TileInfo {
    TileInfo {
      coords: [
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
      ],
      size: Default::default(),
      tile_set_id: Default::default(),
    }
  }
}

#[cfg(test)]
#[allow(unstable)]
mod tests{
  use super::TileInfo;
  use rsfml::system::Vector2f;
  use std::default::Default;

  #[test]
  #[allow(non_snake_case)]
  fn test_explicitly_initialized_TileInfo() {
    let tile_info = TileInfo {
      coords: [
        Vector2f{ x:0., y:1. },
        Vector2f{ x:1., y:2. },
        Vector2f{ x:3., y:4. },
        Vector2f{ x:4., y:5. },
      ],
      size: Vector2f{ x: 100., y: 200. },
      tile_set_id: 15_u16
    };

    assert!(tile_info.coords[0] == Vector2f{ x:0., y:1. });
    assert!(tile_info.coords[1] == Vector2f{ x:1., y:2. });
    assert!(tile_info.coords[2] == Vector2f{ x:3., y:4. });
    assert!(tile_info.coords[3] == Vector2f{ x:4., y:5. });
    assert!(tile_info.size == Vector2f{ x: 100., y: 200. });
    assert!(tile_info.tile_set_id == 15_u16);
  }

  #[test]
  fn test_default_TileInfo() {
    let tile_info: TileInfo = Default::default();
    assert!(tile_info.coords[0] == Vector2f{ x:0., y:0. });
    assert!(tile_info.coords[1] == Vector2f{ x:0., y:0. });
    assert!(tile_info.coords[2] == Vector2f{ x:0., y:0. });
    assert!(tile_info.coords[3] == Vector2f{ x:0., y:0. });
  }

  #[test]
  fn test_field_override_default_TileInfo() {
    let tile_info = TileInfo {
      size: Vector2f{ x: 22., y: 33. },
      ..Default::default()
    };
    assert!(tile_info.size == Vector2f{ x: 22., y: 33. });
  }
}
