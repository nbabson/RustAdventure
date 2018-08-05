mod util;
use util::*; 
use std::collections::HashMap;

fn make_game_objects(objects: &mut HashMap<String, Item>) {
     let hat: Item = Item::new("old fedora".to_string(), 2.0);
     objects.insert("hat".to_string(), hat);
     objects.insert("crystal ball".to_string(), Item::new("grapefruit sized sparkling crystaline orb".to_string(), 6.5));
     
     println!("{:?}", objects);
}


fn make_world(world: &mut Vec<Location>, objects: &mut HashMap<String, Item>) {

}

 #[test]
 fn insert_to_object_map() {
     let mut objects: HashMap<String, Item> = HashMap::new();
     make_game_objects(&mut objects);
     assert_eq!(true, objects.contains_key("hat"));
     let i = objects.get("hat").unwrap();
     assert_eq!(i.description, "old fedora".to_string());
     let w = objects.get("crystal ball").unwrap();
     assert_eq!(6.5, w.weight);
 }


fn main() {
    let mut world: Vec<Location> = Vec::new(); 
    let mut objects: HashMap<String, Item> = HashMap::new();
    make_game_objects(&mut objects);
    make_world(&mut world, &mut objects);
    let l: Location;
    test();
}
