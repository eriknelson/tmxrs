
extern crate rsfml;

pub use self::map_loader::MapLoader;
pub use self::map_layer::MapLayer;
pub use self::map_object::MapObject;

pub enum MapOrientation {
  Orthogonal,
  Isometric,
  SteppedIsometric,
}

mod map_loader;
mod map_layer;
mod map_object;
