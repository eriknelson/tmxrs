use std::collections::HashMap;
use std::default::Default;

//use core::num::from_i32;

use rsfml::system::Vector2f;
use rsfml::graphics::{
  IntRect
};

use super::MapOrientation;

pub struct MapLoader {
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
}

// Holds texture coords and tileset id of a tile
struct TileInfo {
  coords: [Vector2f; 4],
  size: Vector2f,
  tile_set_id: u16,

}

impl TileInfo {
  fn defaultNew() -> TileInfo {
    TileInfo {
      coords: [Vector2f::new(0., 0.); 4],
      size: Vector2f::new(0., 0.),
      tile_set_id: 0_u16,
    }
  }
  fn new(rect: &IntRect,
         size: &Vector2f,
         tile_set_id: u16) -> TileInfo
  {
    //coords: [Vector2f::new(rect.left, rect.top)]
    TileInfo::defaultNew()
  }
}

#[cfg(test)]
mod tests{
  use super::TileInfo;

  #[test]
  fn hello_test_loader() {
    println!("Logging");
    assert!(true);
  }
}
