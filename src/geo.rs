use super::math;
use rsfml::system::Vector2f;

#[derive(Default)]
pub struct Segment {
  pub start: Vector2f,
  pub end: Vector2f,
}

impl Segment {
  pub fn intersects(&self, segment: &Segment) -> bool {
    return math::intersects(self, segment);
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use rsfml::system::Vector2f;
  use super::Segment;

  #[test]
  fn test_segment_default() {
    let seg: Segment = Default::default();
    assert!(seg.start == Default::default());
    assert!(seg.end == Default::default());
  }

  #[test]
  fn test_segment_method_should_intersect(){
    let seg_one = Segment {
      start: Vector2f { x: -3.0, y: 1.0 },
      end: Vector2f { x: 1.0, y: -2.0}
    };

    let seg_two = Segment {
      start: Vector2f { x: 2.0, y: 2.0 },
      end: Vector2f { x: -1.0, y: -4.0 }
    };

    assert!(seg_one.intersects(&seg_two));
    assert!(seg_two.intersects(&seg_one));
  }

  #[test]
  fn test_segment_method_should_not_intersect(){
    let seg_one = Segment {
      start: Vector2f { x: -3.0, y: 1.0 },
      end: Vector2f { x: 1.0, y: -2.0}
    };

    let seg_two = Segment {
      start: Vector2f { x: 2.0, y: 2.0 },
      end: Vector2f { x: 2.0, y: -2.0 }
    };

    assert!(!seg_one.intersects(&seg_two));
    assert!(!seg_two.intersects(&seg_one));
  }

}
