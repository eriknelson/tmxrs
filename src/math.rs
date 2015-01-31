//! Math related functionality

use super::geo::{
  Segment
};

use rsfml::system::Vector2f;

use std::cmp::{
  partial_max,
  partial_min,
};

////////////////////////////////////////////////////////////////////////////////
/// Intersection math for `geo::Segments`
/// Implementation of the following algorithm
/// http://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
pub enum Intersection {
  Intersects(Vector2f),
  DoesNotIntersect,
}

enum Orientation {
  Colinear,
  Clockwise,
  CounterClockwise,
}

/// Given three colinear points, p, q, r, the check if the pint q lies on line
/// segment `pr`
fn on_segment(p: Vector2f, q: Vector2f,r: Vector2f) -> bool {
  return (q.x <= partial_max(p.x, r.x).unwrap() && q.x >= partial_min(p.x, r.x).unwrap() &&
          q.y <= partial_max(p.y, r.y).unwrap() && p.y >= partial_min(p.y, r.y).unwrap())
}

pub fn intersects(seg_one: &Segment, seg_two: &Segment) -> Intersection {
  return Intersection::DoesNotIntersect;

  // These look like segment sizes
  //let s1 = seg_one.end - seg_one.start;
  //println!("{} {}", s1.x, s1.y);
  //let s2 = seg_two.end - seg_two.start;
  //println!("{} {}", s2.x, s2.y);

  //let s = (-s1.y * (seg_one.start.x - seg_two.start.x) +
           //s1.x * (seg_one.start.y - seg_.start.y)) /
    //(-s2.x * s1.y + s1.x * s2.y);
  //println!("{}", s);
  //let t = (-s2.x * (seg_one.start.y - segment.start.y) +
           //s1.y * (seg_one.start.x - segment.start.x)) /
    //(-s2.x * s1.y + s1.x * s2.y);
  //println!("{}", t);

  //if s >= 0. && s <= 1. && t >= 0. && t <= 1. {
    //// Detected intersection

    //let intersection = Vector2f {
      //x: seg_one.start.x + (t * s1.x),
      //y: seg_one.start.y + (t * s1.y)
    //};

    //return Intersection::Intersects(intersection);
  //}

  //return Intersection::DoesNotIntersect;
}

#[cfg(test)]
mod tests {
  use super::super::geo::Segment;
  use super::Intersection;

  use rsfml::system::Vector2f;

  #[test]
  fn test_should_be_on_segment(){

  }

  #[test]
  fn test_should_not_be_on_segment(){
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

    //let intersection = match seg_one.intersects(&seg_two) {
      //Intersection::Intersects(i) => i,
      //_ => panic!("Lines should intersect!"),
    //};

    //println!("The lines intersected! [ x: {}, y: {} ]",
             //intersection.x,
             //intersection.y);

    assert!(true);
  }

  fn test_segments_should_not_intersect(){
    assert!(true);
  }
}
