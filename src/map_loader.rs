use super::MapOrientation;
use std::collections::HashMap;

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
