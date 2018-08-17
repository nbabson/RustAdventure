/* Neil Babson
*  August 2018
*/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod util;
mod locations;
use util::*; 
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write, BufRead};
use std::process::exit;
use util::Direction::*;
use util::MAX_WEIGHT;
use locations::{make_game_objects, make_world};
use util::Visibility::*;
use std::process::Command;

/// Create HashSet of words that are recognized by the game
fn make_dictionary() -> HashSet<&'static str> {
    let dict: HashSet<&str> = [ "q", "quit", "go", "hat", "fedora", "crystal", "ball",
                 "s", "south", "w", "west", "e", "east", "n", "north", "ne", "nw", "se",
                 "sw", "i", "inventory", "l", "look", "the", "score", "at", "in", "get", "take",
                 "northeast", "northwest", "southeast", "southwest", "d", "drop", "up", "down",
                 "laptop", "key", "birds", "nest", "climb", "ladder", "statue"].iter().cloned().collect();
    dict    
}

/// Create a new game player
fn make_player(objects: &mut HashMap<String, Item>) -> Player {
    Command::new("clear").status().unwrap(); //spawn().expect("");
    print!("Welcome! What is your name? ");
    io::stdout().flush().ok().unwrap();
    let stdin = io::stdin();
    let name = stdin.lock().lines().next().unwrap().unwrap();
    let mut starting_inventory: Vec<Item> = Vec::new();
    starting_inventory.push(objects.remove("crystal ball").unwrap());
    Player::new(name, starting_inventory)
}

/// Test that player has been created and given starting inventory object
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

/// Get a new command input from the player and verify that it contains only
/// valid words before passing the vector of valid words to the parser
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

/// Display the current score and number of turns that have passed
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

/// Verify that a player wants to quit the game
fn quit(player: &Player) {
    let stdin = io::stdin();
    println!("Are you sure you want to quit? ('no' to continue)");     
    let answer = stdin.lock().lines().next().unwrap().unwrap();
    if answer.trim() != "no".to_string() {
        score(player);
        exit(0);
    }
}

/// Display the player's inventory
fn inventory(player: &Player) {
    if player.inventory.len() == 0 {
        println!("You have no items.");
    }
    else {
        println!("You are carrying the following items:");
        for item in &player.inventory {
            println!("\t{}", item.name);
        }
    }
}

/// See if it is possible for the player to move in the direction they have chosen.
/// If so return new Location index, otherwise return old Location index.
fn try_to_move(world: &Vec<Location>, dir: Direction, old_location: usize) -> usize {
    let mut exit_exists: bool = false;
    for exit in &world[old_location].exits {
        if dir == exit.direction {
            if exit.locked {  println!("That exit is locked."); }
            else { return exit.goes_to; }
            exit_exists = true;
        }
    }
    if !exit_exists { println!("You can't go that way."); }
    old_location
}

/// Parse a vector of valid game words to try and interpret it as a valid game command.
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
        "l" | "look"          => { look(world, location_index, command, command_index, player); }, 
        "get" | "take"        => { get(world, location_index, command, command_index, player); },
        "d" | "drop"          => { drop(world, location_index, command, command_index, player); },
        "n" | "north"         => { new_location = try_to_move(world, N, location_index); },
        "s" | "south"         => { new_location = try_to_move(world, S, location_index); },
        "w" | "west"          => { new_location = try_to_move(world, W, location_index); },
        "e" | "east"          => { new_location = try_to_move(world, E, location_index); },
        "ne" | "northeast"    => { new_location = try_to_move(world, NE, location_index); },
        "se" | "southeast"    => { new_location = try_to_move(world, SE, location_index); },
        "nw" | "northwest"    => { new_location = try_to_move(world, NW, location_index); },
        "sw" | "southwest"    => { new_location = try_to_move(world, SW, location_index); },
        "up"                  => { new_location = try_to_move(world, U, location_index); },
        "down"                => { new_location = try_to_move(world, D, location_index); },
        "climb"               => { new_location = climb(world, location_index, command, command_index); }
         _                    => {},
    }  
    new_location
}

/// Try to climb to a different location. If direction (up or down) is not specified go on the direction possible for
/// climbable object (currently only ladder)
fn climb(world: &Vec<Location>, old_location: usize, command: Vec<String>, mut command_index: usize) -> usize {
    command_index += 1;
    let mut new_location = old_location;
    let mut up_or_down = 0;
    if command_index == command.len() {
        println!("I don't understand.");
        return old_location;
    }
    if command[command_index] == "the" { 
        command_index += 1;
    }
    if command_index < command.len() {
         match command[command_index].as_str() {
             "up"           => { new_location = try_to_move(world, U, old_location); },
             "down"         => { new_location = try_to_move(world, D, old_location); },
             "ladder"       => { for exit in &world[old_location].exits {
                                      match exit.direction {
                                          U    => { up_or_down = 1; },
                                          D    => { up_or_down = -1; },
                                          _    => {},
                                      }
                                  }
                                  if up_or_down == 1 {  new_location = try_to_move(world, U, old_location); }
                                  else if up_or_down == -1 { new_location = try_to_move(world, D, old_location); }
                                }               
                  _            => { println!("You can't climb that."); }, 
         }
    }
    new_location
}    

/// Try to drop an object, removing it from the player's inventory and adding it to the Item vector of the current Location.
fn drop(mut world: &mut Vec<Location>, location_index: usize, command: Vec<String>, mut command_index: usize, mut player: &mut Player) {
    command_index += 1;
    if command_index < command.len() {
        if command[command_index] == "the" { command_index += 1; }
        let mut found: bool = false;
        let mut found_at: usize = 0; 
        for i in 0..player.inventory.len() {
            let mut item = &player.inventory[i];
            if command.len() > command_index && command[command_index] == item.name {
                found = true;
                found_at = i;
                println!("You drop {}.", item.name);
                player.weight -= item.weight;
            }
            else if command.len() > command_index + 1 && (format!("{} {}", &command[command_index],&command[command_index + 1])
                        == item.name) {
                found = true;
                found_at = i;
                println!("You drop the {}.", item.name);
                player.weight -= item.weight;
            }
        }
        if found {
            let object = player.inventory.remove(found_at);           
            world[location_index].items.push(object);
            return;
        } 
    }
    println!("You do not have that.");
}

/// Try to get an object from the Item vector of the location and add it to the player's inventory vector.
/// If the Item's weight would increase the player's carried weight past MAX_WEIGHT the object can not be picked up.
fn get(mut world: &mut Vec<Location>, location_index: usize, command: Vec<String>, mut command_index: usize, mut player: &mut Player) {
    command_index += 1;
    if command_index < command.len() {
        if command[command_index] == "the" { command_index += 1; }
        let mut found: bool = false;
        let mut found_at: usize = 0; 
        for i in 0..world[location_index].items.len() {
            let mut item = &world[location_index].items[i];
            if command.len() > command_index && command[command_index] == item.name {
                found = true;
                found_at = i;
                if player.weight + item.weight > MAX_WEIGHT {
                    println!("You are overencumbered and can not take the {}.", item.name);
                    return;
                }
                player.weight += item.weight;
                println!("You take the {}.", item.name);
            }
            else if command.len() > command_index + 1 && (format!("{} {}", &command[command_index],&command[command_index + 1])
                        == item.name) {
                found = true;
                found_at = i;
                if player.weight + item.weight > MAX_WEIGHT {
                    println!("You are overencumbered and can not take the {}.", item.name);
                    return;
                }
                player.weight += item.weight;
                println!("You take the {}.", item.name);
            }
        }
        if found {
            let object = world[location_index].items.remove(found_at);
            if object.contains != None {
               // get contained object
            }
            player.inventory.push(object);            

            return;
        } 
        for item in &player.inventory {
            if (command.len() > command_index && command[command_index] == item.name) ||
                        (command.len() > command_index + 1 && (format!("{} {}", &command[command_index],&command[command_index + 1])
                        == item.name)) {
                println!("You already have the {}.", item.name);
                return;
            }
        }
    }
    println!("You do not see that here.");
}

/// Look. By itself this command gives a description of the current location including items present (those which aren't Hidden).
/// Followed by the name of an item (optionally preceded by 'at') this command gives the description of an object present
/// at the location or carried by the player. Following 'look' by 'in' lets the player look in objects that contain other objects,
/// revealing the presence of Hidden Items.
fn look(world: &Vec<Location>, location_index: usize, command: Vec<String>, mut command_index: usize, mut player: &mut Player) {
    command_index += 1;
    if command.len() <= command_index {
        println!("\n{}", world[location_index].description);
        for item in &world[location_index].items {
            if item.visibility != Hidden {
                print!("There is a");
                match item.name.char_indices().next().unwrap() {
                    (0, 'a') | (0, 'e') | (0, 'i') | (0, 'o') | (0, 'u') => { print!("n"); },
                     _                                                   => {},
                }       
                println!(" {} here.", item.name);
            }
        }
        for exit in &world[location_index].exits {
            println!("You see {}.", exit.description);
        }
        println!();
    }
    else {
        if command[command_index] == "in" {
            command_index += 1;
            if command_index < command.len() &&  command[command_index] == "the" { command_index += 1; }
            for item in &world[location_index].items {
                if (command.len() > command_index && command[command_index] == item.name) || (command.len() > command_index + 1
                        && (format!("{} {}", &command[command_index],&command[command_index + 1]) == item.name)) {
                    println!("You do not have the {}.", item.name);
                    return;
                }
            }
            for i in 0..player.inventory.len() {
                if player.inventory[i].contains != None {                   
                     for j in 0..player.inventory.len() {
                         if &player.inventory[i].contains.as_ref() == &Some(&player.inventory[j].name) {
                              println!("There is a {} here.", player.inventory[j].name);
                              println!("You see {}.", player.inventory[j].description);
                              player.inventory[j].visibility = Seen;
                              return;
                          }
                     }
                }
            }
            println!("You do not see that here.");
            return;
        }
        else if command[command_index] == "at" { command_index += 1; }  
        if command_index < command.len() &&  command[command_index] == "the" { command_index += 1; }
        for item in &world[location_index].items {
            if (command.len() > command_index && command[command_index] == item.name) ||
                        (command.len() > command_index + 1 && (format!("{} {}", &command[command_index],&command[command_index + 1])
                        == item.name)) {
                println!("You see {}.", item.description);
                return;
            }
        }
        for item in &player.inventory {
            if (command.len() > command_index && command[command_index] == item.name) ||
                        (command.len() > command_index + 1 && (format!("{} {}", &command[command_index],&command[command_index + 1])
                        == item.name)) {
                println!("You see {}.", item.description);
                return;
            }
        }
        println!("You do not see that here.");
    }    
}

/// Display information about the current location.
/// Get and parse player's commands until they move to a different Location.
fn play_game(mut world: &mut Vec<Location>, mut player: &mut Player, location_index: usize, dictionary: &HashSet<&str>) ->usize {
    println!("\n{}", world[location_index].name);
    let mut next_location = location_index;
    if !world[location_index].visited {
        let v: Vec<String> = Vec::new();
        look(world, location_index, v, 0, player);
        world[location_index].visited = true;
    }
    while next_location == location_index {
        // Command is an unparsed vector of legal game words
        let command: Vec<String> = get_command(&mut player, &dictionary);
        next_location = parse_command(command, &mut world, &mut player, location_index);
    }
    next_location
}

/// Create HashMap of game objects, Location vector of game Locations, HashSet of allowable game words,
/// and a new player. Starting with initial Location with index 1, call play_game in infinite loop
/// passing in the new Location index.
fn main() {
    let mut objects: HashMap<String, Item> = HashMap::new();
    let mut world: Vec<Location> = Vec::new(); 
    let dictionary: HashSet<&str> = make_dictionary();
    make_game_objects(&mut objects);
    make_world(&mut world, &mut objects);
    let mut player = make_player(&mut objects);
    let mut next_location = 1;

    loop {
        next_location = play_game(&mut world, &mut  player, next_location, &dictionary); 
    }

}

