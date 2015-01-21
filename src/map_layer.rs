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
