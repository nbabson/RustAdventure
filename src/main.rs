/* Neil Babson
*  August 2018
*/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod util;
use util::*; 
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write, BufRead};


fn make_dictionary() -> HashSet<&'static str> {
    let dict: HashSet<&str> = [ "go", "n", "north", "hat", "fedora", "crystal", "ball",
                 "s", "south", "w", "west", ].iter().cloned().collect();
    dict    
}


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

// This function contains various failed attempts to put vector of strings into lowercase
// and not lose ownership of it. These will be cleaned up. 
fn get_command<'a,'b>(player: &'a mut Player, dictionary: &HashSet<&str>) -> Vec<String> {
    let stdin = io::stdin();
    let mut success = false;

    while !success { 
        success = true;
        if player.turns < 5 {
            print!("What would you like to do? ");
        }
        else {
            print!("> ");
        }
        io::stdout().flush().ok().unwrap();
        let command = stdin.lock().lines().next().unwrap().unwrap();
        let words: Vec<String> = command.split(' ').collect::<Vec<&str>>().
                      iter().map(|x| x.to_lowercase()).collect();
        let len = words.len();
        if len == 1 && words[0] == "" {
            println!("I didn't catch that.");
            success = false;
        }
        else {
            for i in 0..len {
                if !dictionary.contains(words[i].as_str()) {
                    success = false;
                    println!("Sorry I don't know the word {}.", words[i]);
                }
            }
        }
        if  success {
                player.turns += 1;    
                return words; }
    } 
    vec!["Error".to_string()]
}


fn parse_command(command: Vec<String>, mut world: &mut Vec<Location>, 
        mut player: &mut Player, location_index: usize) -> usize {

    0
}

fn play_game(mut world: &mut Vec<Location>, mut player: &mut Player, location_index: usize, dictionary: &HashSet<&str>) ->usize {
    println!("\n{}\n", world[location_index].name);
    let mut next_location = location_index;
    if !world[location_index].visited {
        println!("{}", world[location_index].description);
    }
    world[location_index].visited = true;
    for item in &world[location_index].items {
        print!("There is a");
        match item.name.char_indices().next().unwrap() {
            (0, 'a') | (0, 'e') | (0, 'i') | (0, 'o') | (0, 'u') => { print!("n"); },
             _                                                   => {},
        }       
        println!(" {} here.", item.name);
    }
    while next_location == location_index {
        println!("\n{}\n", world[location_index].name);
        // Command is an unparsed vector of legal game words
        let command: Vec<String> = get_command(&mut player, &dictionary);
        next_location = parse_command(command, &mut world, &mut player, location_index);
    }
    next_location
}

fn main() {
    let mut world: Vec<Location> = Vec::new(); 
    let mut objects: HashMap<String, Item> = HashMap::new();
    let dictionary: HashSet<&str> = make_dictionary();
    make_game_objects(&mut objects);
    make_world(&mut world, &mut objects);
    let mut player = make_player(&mut objects);

    let next_location = play_game(&mut world, &mut  player, 0, &dictionary); 

}

