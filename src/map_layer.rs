// TODO: Blocked on MapObject

use std::default::Default;
use std::rc::Rc;
use std::num::NumCast;
use std::vec::Vec;
use rsfml::system::Vector2f;
use rsfml::graphics::{
  Texture,
  Vertex,
  FloatRect,
  RenderTarget,
  RenderStates
};

pub struct TileQuad {
  indices: [u16; 4],
  movement: Vector2f,
  needs_update: bool,
}

impl Default for TileQuad {
  fn default() -> TileQuad {
    TileQuad {
      indices: [0, 0, 0, 0],
      movement: Default::default(),
      needs_update: false,
    }
  }
}
impl TileQuad {
  pub fn new() -> TileQuad{ Default::default() }
  pub fn new_w_indices(indices: [u16; 4]) -> TileQuad{
    TileQuad {
      indices: indices, ..Default::default()
    }
  }

  pub fn say_hi() { println!("Hello"); }

  pub fn move_distance(&mut self, distance: &Vector2f) {
    self.movement = distance.clone();
    self.needs_update = true;
  }
}

pub struct LayerSet<'a> {
  texture: &'a Texture,
  quads: Vec<Rc<TileQuad>>,
  vertices: Vec<Vertex>,
  bounding_box: FloatRect,
  is_visible: bool,
}

impl<'a> LayerSet<'a> {
  pub fn new(texture: &Texture) -> LayerSet {
    LayerSet {
      texture: texture,
      quads: Vec::new(),
      vertices: Vec::new(),
      bounding_box: FloatRect {
        left: 0., top: 0., width: 0., height: 0.
      },
      is_visible: true
    }
  }

  pub fn add_tile(
    &mut self, v0: Vertex, v1: Vertex, v2: Vertex, v3: Vertex) -> Rc<TileQuad>
  {
    self.vertices.push(v0);
    self.vertices.push(v1);
    self.vertices.push(v2);
    self.vertices.push(v3);

    let u16len: u16 = NumCast::from(self.vertices.len()).unwrap();
    let i = u16len - 4_u16;

    self.quads.push(
      Rc::new(TileQuad::new_w_indices([i, i+1, i+2, i+3]))
    );
    //self.update_aabb(vertices[0], vertices[2]);
    match self.quads.last() {
      Some(quad) => quad.clone(),
      _ => panic!("Quads of len 0!")
    }
  }
  //pub fn cull(&self, bounds: &FloatRect) {
  //}

  // Private
  //fn draw(&self, rt: &RenderTarget, states: &RenderStates) {
  //}
  //fn update_aabb(&self, position: &Vector2f, size) {
  //}
}

pub struct MapLayer {
  pub name: String
}

impl MapLayer {
  pub fn new(name: &str) -> MapLayer {

    MapLayer {
      name: name.to_string()
    }

  }
}

#[cfg(test)]
mod tests {
  use super::TileQuad;
  use rsfml::system::Vector2f;
  use std::default::Default;

  #[test]
  fn test_tile_quad_new() {
    let t = TileQuad::new();

    assert!(t.indices.len() == 4);
    assert!(t.indices[0] == 0);
    assert!(t.indices[1] == 0);
    assert!(t.indices[2] == 0);
    assert!(t.indices[3] == 0);

    assert!(t.movement.x == 0.);
    assert!(t.movement.y == 0.);

    assert!(t.needs_update == false);

  }

  #[test]
  fn test_tile_quad_new_w_indices() {
    let t = TileQuad::new_w_indices([1, 3, 3, 7]);

    assert!(t.indices.len() == 4);
    assert!(t.indices[0] == 1);
    assert!(t.indices[1] == 3);
    assert!(t.indices[2] == 3);
    assert!(t.indices[3] == 7);

    assert!(t.movement.x == 0.);
    assert!(t.movement.y == 0.);

    assert!(t.needs_update == false);

  }

  #[test]
  fn test_tile_quad_move_distance() {
    let mut t: TileQuad = TileQuad::new();

    t.move_distance(&Vector2f{ x: 2.0, y: 5.0 });

    assert!(t.movement.x == 2.0);
    assert!(t.movement.y == 5.0);
    assert!(t.needs_update == true);
  }
}
