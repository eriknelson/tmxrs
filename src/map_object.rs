use super::geo::{
  Segment
};
use rsfml::system::Vector2f;
use std::vec::Vec;
use std::collections::{
  HashMap,
};

pub enum MapObjectShape
{
  Rectangle,
  Ellipse,
  Circle,
  Polygon,
  Polyline,
  Tile
}
pub struct MapObject {
  name: String,
  object_type: String,
  parent: String, // name of layer to which object belongs
  position: Vector2f,
  size: Vector2f,
  properties: HashMap<String, String>,
  visible: bool,
  // List of points defining any polygonal shape
  polypoints: Vec<Vector2f>,
  shape: MapObjectShape,
  //debug_shape: DebugShape, // TODO: Not sure what this is
  center_point: Vector2f,
  poly_segs: Vec<Segment>,
  //tile_quad: Box<TileQuad>
}
