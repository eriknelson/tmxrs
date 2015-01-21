extern crate tmxrs;

#[test]
fn test_map_layer() {
  let first_map_name = "first map";
  let second_map_name = "second map";
  let ml_one = tmxrs::MapLayer::new(first_map_name);
  let ml_two = tmxrs::MapLayer::new(second_map_name);

  assert_eq!(ml_one.name, first_map_name);
  assert!(ml_two.name == second_map_name);
}
