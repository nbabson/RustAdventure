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
use std::process::exit;
use util::Direction::*;

fn make_dictionary() -> HashSet<&'static str> {
    let dict: HashSet<&str> = [ "q", "quit", "go", "hat", "fedora", "crystal", "ball",
                 "s", "south", "w", "west", "e", "east", "n", "north", "ne", "nw", "se",
                 "sw", "inventory", "l", "look", "the", "score"].iter().cloned().collect();
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
    exits.push(Exit::new("a low archway to the south".to_string(), 1, Direction::S, false));
    exits.push(Exit::new("a ladder in the northeast corner".to_string(), 2, Direction::NE, false)); 
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
    let name = "Test".to_string();
    let mut starting_inventory: Vec<Item> = Vec::new();
    starting_inventory.push(o.remove("crystal ball").unwrap());
    let p = Player::new(name, starting_inventory);
    let ball: Item = Item::new("crystal ball".to_string(), "grapefruit sized sparkling crystalline orb".to_string(), 6.5);
    assert_eq!(true, p.inventory.contains(&ball));
}

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

fn score(player: &Player) {
    print!("\nYour score is {} points after {} ", player.score, player.turns);
    if player.score > 1 { println!("turns."); }
    else { println!("turn."); }
    let rank = match player.score {
        0...10 => "beginner".to_string(),
        _     => "expert".to_string(),
    };    
    println!("Your rank is {}.", rank);    
}

fn quit(player: &Player) {
    let stdin = io::stdin();
    println!("Are you sure you want to quit? ('no' to continue)");     
    let answer = stdin.lock().lines().next().unwrap().unwrap();
    if answer.trim() != "no".to_string() {
        score(player);
        exit(0);
    }
}

fn inventory(player: &Player) {
    if player.inventory.len() == 0 {
        println!("You have no items.");
    }
    else {
        println!("You are carrying the following items");
        for item in &player.inventory {
            println!("\t{}", item.name);
        }
    }
}

fn try_to_move(world: &Vec<Location>, dir: Direction, old_location: usize) -> usize {

    let mut exit_exists: bool = false;
    for loc in world {
        for exit in &loc.exits {
            if dir == exit.direction {
                if exit.locked {  println!("That exit is locked."); }
                else { return exit.goes_to; }
                exit_exists = true;
            }
        }
    }
    if !exit_exists { println!("You can't go that way."); }
    old_location
}

fn parse_command(command: Vec<String>, mut world: &mut Vec<Location>, 
        mut player: &mut Player, location_index: usize) -> usize {

    let mut command_index = 0;
    let mut new_location = location_index;
    while command[command_index] == "the" || command[command_index] == "go" {
        command_index += 1;
    }
    match command[command_index].as_str() {
        "q" | "quit"          => { quit(player); },
        "score"               => { score(player); },
        "i" | "inventory"     => { inventory(player); },
        "l" | "look"          => { look(world, location_index); }, // check for look at
        "n" | "north"         => { new_location = try_to_move(world, N, location_index); },
        "s" | "south"         => { new_location = try_to_move(world, S, location_index); },
        "w" | "west"          => { new_location = try_to_move(world, W, location_index); },
        "e" | "east"          => { new_location = try_to_move(world, E, location_index); },
        "ne"                  => { new_location = try_to_move(world, NE, location_index); },
        "se"                  => { new_location = try_to_move(world, SE, location_index); },
        "nw"                  => { new_location = try_to_move(world, NW, location_index); },
        "sw"                  => { new_location = try_to_move(world, SW, location_index); },
         _                    => {},
    }  
    new_location
}

fn look(world: &Vec<Location>, location_index: usize) {
    println!("\n{}", world[location_index].description);
    for item in &world[location_index].items {
        print!("There is a");
        match item.name.char_indices().next().unwrap() {
            (0, 'a') | (0, 'e') | (0, 'i') | (0, 'o') | (0, 'u') => { print!("n"); },
             _                                                   => {},
        }       
        println!(" {} here.", item.name);
    }
    for exit in &world[location_index].exits {
        println!("You see {}.", exit.description);
    }
}

fn play_game(mut world: &mut Vec<Location>, mut player: &mut Player, location_index: usize, dictionary: &HashSet<&str>) ->usize {
    println!("\n{}\n", world[location_index].name);
    let mut next_location = location_index;
    if !world[location_index].visited {
        look(world, location_index);
        world[location_index].visited = true;
    }
    while next_location == location_index {
        //println!("\n{}\n", world[location_index].name);
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

