//! Math related functionality

use super::geo::{
  Segment
};

use rsfml::system::Vector2f;

use std::cmp::{
  partial_max,
  partial_min,
};

#[derive(PartialEq)]
enum Orientation {
  Collinear,
  Clockwise,
  CounterClockwise,
}

/// Given three collinear points, p, q, r, the check if the pint q lies on line
/// segment `pr`
fn on_segment(p: Vector2f, q: Vector2f, r: Vector2f) -> bool {
  return (q.x <= partial_max(p.x, r.x).unwrap() && q.x >= partial_min(p.x, r.x).unwrap() &&
          q.y <= partial_max(p.y, r.y).unwrap() && p.y >= partial_min(p.y, r.y).unwrap())
}

fn get_orientation(p: &Vector2f, q: &Vector2f, r: &Vector2f) -> Orientation {
  let val = (q.y - p.y) * (r.x - q.x) -
            (q.x - p.x) * (r.y - q.y);

  return if val == 0.     { Orientation::Collinear }
         else if val > 0. { Orientation::Clockwise }
         else             { Orientation::CounterClockwise }
}

/// Intersection math for `geo::Segments`
/// Implementation of the following algorithm
/// http://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
///
/// Returns `true` if line seg_one and seg_two intersect
pub fn intersects(seg_one: &Segment, seg_two: &Segment) -> bool {

  // Find the four orientations needed for general and special cases
  let o1 = get_orientation(&seg_one.start, &seg_one.end, &seg_two.start);
  let o2 = get_orientation(&seg_one.start, &seg_one.end, &seg_two.end);
  let o3 = get_orientation(&seg_two.start, &seg_two.end, &seg_one.start);
  let o4 = get_orientation(&seg_two.start, &seg_two.end, &seg_one.end);

  // General case
  if o1 != o2 && o3 != o4 { return true; };

  // Special Cases
  if o1 == Orientation::Collinear &&
     on_segment(seg_one.start, seg_two.start, seg_one.end){ return true; };
  if o2 == Orientation::Collinear &&
     on_segment(seg_one.start, seg_two.end, seg_one.end){ return true; };
  if o3 == Orientation::Collinear &&
     on_segment(seg_two.start, seg_one.start, seg_two.end){ return true; };
  if o4 == Orientation::Collinear &&
     on_segment(seg_two.start, seg_one.end, seg_two.end){ return true; };

  return false;
}

#[cfg(test)]
mod tests {
  use super::super::geo::Segment;
  use super::{
    on_segment,
    intersects,
    Orientation,
    get_orientation,
  };

  use rsfml::system::Vector2f;

  #[test]
  fn test_should_be_on_segment(){
    let forward_on_segment = on_segment(
      Vector2f { x: 1., y: 1. },
      Vector2f { x: 2., y: 2. },
      Vector2f { x: 3., y: 3. },
    );
    assert!(forward_on_segment);

    let reverse_on_segment = on_segment(
      Vector2f { x: 3., y: 3. },
      Vector2f { x: 2., y: 2. },
      Vector2f { x: 1., y: 1. },
    );
    assert!(reverse_on_segment);
  }

  #[test]
  fn test_should_not_be_on_segment(){
    let not_on_segment = on_segment(
      Vector2f { x: 1., y: 1. },
      Vector2f { x: 4., y: 4. },
      Vector2f { x: 3., y: 3. },
    );
    assert!(!not_on_segment);
  }

  #[test]
  fn test_orientation_collinear() {
    let a = Vector2f { x: 1., y: 1. };
    let b = Vector2f { x: 0.4, y: 0.4 };
    let c = Vector2f { x: 3., y: 3. };

    match get_orientation(&a, &b, &c) {
      Orientation::Collinear => assert!(true),
      _ => panic!("Expectation failed!"),
    };
  }

  #[test]
  fn test_orientation_clockwise() {
    let a = Vector2f { x: 1., y: 2. };
    let b = Vector2f { x: -2., y: -2. };
    let c = Vector2f { x: 2., y: -2. };

    match get_orientation(&a, &b, &c) {
      Orientation::CounterClockwise => assert!(true),
      _ => panic!("Expectation failed!"),
    };

  }

  #[test]
  fn test_orientation_counter_clockwise() {
    let a = Vector2f { x: 2., y: -2. };
    let b = Vector2f { x: -2., y: -2. };
    let c = Vector2f { x: 1., y: 2. };

    match get_orientation(&a, &b, &c) {
      Orientation::Clockwise => assert!(true),
      _ => panic!("Expectation failed!"),
    };

  }

  #[test]
  fn test_segments_should_intersect(){
    let seg_one = Segment {
      start: Vector2f { x: -3.0, y: 1.0 },
      end: Vector2f { x: 1.0, y: -2.0}
    };

    let seg_two = Segment {
      start: Vector2f { x: 2.0, y: 2.0 },
      end: Vector2f { x: -1.0, y: -4.0 }
    };

    assert!(intersects(&seg_one, &seg_two));
  }

  #[test]
  fn test_segments_should_not_intersect(){
    let seg_one = Segment {
      start: Vector2f { x: -3.0, y: 1.0 },
      end: Vector2f { x: 1.0, y: -2.0}
    };

    let seg_two = Segment {
      start: Vector2f { x: 2.0, y: 2.0 },
      end: Vector2f { x: 2.0, y: -2.0 }
    };

    assert!(!intersects(&seg_one, &seg_two));
  }

}
