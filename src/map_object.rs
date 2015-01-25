use std::default::Default;

use rsfml::system::Vector2f;

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
  name: i16, // Stub to build
}

#[derive(Default)]
struct Segment {
  start: Vector2f,
  end: Vector2f,
}

impl Segment {
  fn intersects(segment: &Segment) -> bool {
    return false;
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;

  use super::Segment;

  #[test]
  fn test_segment_default() {
    let seg: Segment = Default::default();
    assert!(seg.start == Default::default());
    assert!(seg.end == Default::default());
  }

  //#[test]
  //fn test_segments_should_intersect() {

  //}

  //#[test]
  //fn test_segments_should_not_intersect() {
  //}

}
