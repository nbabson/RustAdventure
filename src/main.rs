/* Neil Babson
*  August 2018
*/

#![allow(dead_code)]
#![allow(unused_variables)]

mod util;
use util::*; 
use std::collections::HashMap;
use std::io::{self, Write, BufRead};

fn make_game_objects(objects: &mut HashMap<String, Item>) {
     let hat: Item = Item::new("hat".to_string(), "old fedora".to_string(), 2.0);
     objects.insert("hat".to_string(), hat);
     objects.insert("crystal ball".to_string(), 
             Item::new("crystal ball".to_string(), "grapefruit sized sparkling crystalline orb".to_string(), 6.5));
     //println!("{:?}", objects);
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

fn make_world(world: &mut Vec<Location>, objects: &mut HashMap<String, Item>) {
    let mut items: Vec<Item> = Vec::new();
    items.push(objects.remove("hat").unwrap()); 
    let mut exits: Vec<Exit> = Vec::new();
    exits.push(Exit::new("A low archway to the south".to_string(), 1, Direction::S));
    exits.push(Exit::new("A ladder in the northeast corner".to_string(), 2, Direction::NE)); 
    world.push(Location::new(0, "Leafy Courtyard".to_string(), 
                 "This pleasant courtyard is enclosed by vine covered brick walls and contains ".to_string() +
                 "a gaily splashing fountain at its center.", items, false, exits)); 
    //println!("{:?}", world);
}

#[test]
fn object_is_in_location() {
    let mut w: Vec<Location> = Vec::new();
    let mut o: HashMap<String, Item> = HashMap::new();
    make_game_objects(&mut o);
    make_world(&mut w, &mut o);
    let hat: Item = Item::new("hat".to_string(), "old fedora".to_string(), 2.0);
    assert_eq!(true, w[0].items.contains(&hat));
}

fn make_player(objects: &mut HashMap<String, Item>) -> Player {
    print!("Welcome! What is your name? ");
    io::stdout().flush().ok().unwrap();
    let stdin = io::stdin();
    let name = stdin.lock().lines().next().unwrap().unwrap();
    let mut starting_inventory: Vec<Item> = Vec::new();
    starting_inventory.push(objects.remove("crystal ball").unwrap());
    Player::new(name, starting_inventory)
}

#[test]
fn player_has_object() {
    let mut o: HashMap<String, Item> = HashMap::new();
    make_game_objects(&mut o);
    let p = make_player(&mut o);
    println!("Test name");
    io::stdout().flush().ok().unwrap();
    let ball: Item = Item::new("crystal ball".to_string(), "grapefruit sized sparkling crystalline orb".to_string(), 6.5);
    assert_eq!(true, p.inventory.contains(&ball));
}

fn main() {
    let mut world: Vec<Location> = Vec::new(); 
    let mut objects: HashMap<String, Item> = HashMap::new();
    make_game_objects(&mut objects);
    make_world(&mut world, &mut objects);
    let player = make_player(&mut objects);

    let l: Location;
    //test();
}
