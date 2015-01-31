use super::math;
use rsfml::system::Vector2f;

#[derive(Default)]
pub struct Segment {
  pub start: Vector2f,
  pub end: Vector2f,
}

impl Segment {
  pub fn intersects(&self, segment: &Segment) -> math::Intersection {
    return math::Intersection::DoesNotIntersect;
    //return math::intersects(&self, &segment)
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

}
