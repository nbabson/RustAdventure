use util::{Location, Exit, Item};
use std::collections::HashMap;
use util::Visibility::*;
use util::Direction::*;
use util::MAX_WEIGHT;

/// All objects int the game are first created here and placed in a HashMap.
/// As Items are allocated to Locations or Creatures they are removed from the Map.
/// Items have one or two word names.
pub fn make_game_objects(objects: &mut HashMap<String, Item>) {
     let hat: Item = Item::new("hat".to_string(), "a battered old fedora".to_string(), 2.0, None, Visible);
     objects.insert("hat".to_string(), hat);
     objects.insert("crystal ball".to_string(), Item::new("crystal ball".to_string(),
                 "a grapefruit sized sparkling crystalline orb".to_string(), 6.5, None, Visible));
     objects.insert("laptop".to_string(), Item::new("laptop".to_string(), "a small hp laptop with ".to_string() + 
                 "an unfinished Rust program displayed on the screen", 5.0, None, Visible));
     objects.insert("key".to_string(), Item::new("key".to_string(), "a large antique iron key".to_string(),
                 0.5, None, Hidden));
     objects.insert("birds nest".to_string(), Item::new("birds nest".to_string(), "a small bowl of interwoven ".to_string()
                 + "twigs and straw containing fragments of light blue shell", 1.0, Some("key".to_string()), Visible));
     objects.insert("statue".to_string(), Item::new("statue".to_string(), "a small energetic statue depicting two ".to_string() +
                 "horned beetles locked in combat", MAX_WEIGHT, None, Visible));
     objects.insert("egg beater".to_string(), Item::new("egg beater".to_string(), "a somewhat rusty antique mechanical egg beater".to_string(),
                 3.0, None, Visible));
}

/// Test whether objects have been seccessfully placed in object HashMap
#[test]
fn insert_to_object_map() {
 let mut objects: HashMap<String, Item> = HashMap::new();
 make_game_objects(&mut objects);
 assert_eq!(true, objects.contains_key("hat"));
 let i = objects.get("hat").unwrap();
 assert_eq!(i.description, "a battered old fedora".to_string());
 let w = objects.get("crystal ball").unwrap();
 assert_eq!(6.5, w.weight);
}

/// Build all of the game locations as a vector. Locations are accessed by index.
pub fn make_world(world: &mut Vec<Location>, objects: &mut HashMap<String, Item>) {
    // #0 Unfinished Location
    let mut items: Vec<Item> = Vec::new();
    let mut exits: Vec<Exit> = Vec::new();
    items.push(objects.remove("laptop").unwrap());
    world.push(Location::new(0, "Unfinished Location".to_string(), "This unfinished space is ".to_string() +
                "barren and eerily quite.", items, false, exits));
    // #1 Leafy Courtyard
    items = Vec::new();
    exits = Vec::new();
    items.push(objects.remove("hat").unwrap()); 
    exits.push(Exit::new("a low archway to the south".to_string(), 4, S, false));
    exits.push(Exit::new("a ladder in the northeast corner".to_string(), 2, NE, false)); 
    world.push(Location::new(1, "Leafy Courtyard".to_string(), 
                 "This pleasant courtyard is enclosed by vine covered brick walls and contains ".to_string() +
                 "a gaily splashing fountain at its center.", items, false, exits)); 
    // #2 Base of Ladder
    items = Vec::new();
    exits = Vec::new();
    exits.push(Exit::new("a rusty metal ladder attached to the brick wall".to_string(), 3, U, false));
    exits.push(Exit::new("the courtyard to the southwest".to_string(), 1, SW, false));
    world.push(Location::new(2, "Base of Ladder".to_string(), "In this corner of the courtyard you hear ".to_string() +
                "splashing of the fountain as well as a metallic squeaking that seems to come from beyond the wall.",
                items, false, exits));
    // #3 Top of Wall
    items = Vec::new();
    exits = Vec::new();
    exits.push(Exit::new("a ladder extending down into a courtyard".to_string(), 2, D, false));
    exits.push(Exit::new("the top of the wall continuing to the south".to_string(), 0, S, false));
    exits.push(Exit::new("the top of the wall continuing to the north".to_string(), 0, N, false));
    items.push(objects.remove("birds nest").unwrap());
    items.push(objects.remove("key").unwrap());
    world.push(Location::new(3, "Top of Wall".to_string(), "The corner of the crumbling brick wall is somewhat ".to_string() +
                "less than two feet accross. Outside the wall you see a hulking machine that vibrates, squeaks, " +
                "and smokes to no obvious purpose.", items, false, exits));
    // #4 Promonade
    items = Vec::new();
    exits = Vec::new();
    exits.push(Exit::new("a brick archway to the north".to_string(), 1, N, false));
    exits.push(Exit::new("the path continuing to the south".to_string(), 0, S, false));
    items.push(objects.remove("statue").unwrap());
    world.push(Location::new(4, "Promenade".to_string(), "A stately cobbled path processes between rows of stone ".to_string() +
                "columns and weed choked ponds.", items, false, exits));
    // # Damp Tunnel
    items = Vec::new();
    exits = Vec::new();
    exits.push(Exit::new("a wooden ladder ascends to a bright circle above".to_string(), 4, U, false));
    exits.push(Exit::new("to the east the tunnel extends into complete darkness".to_string(), 0, E, true));
    exits.push(Exit::new("to the west the tunnel extends into complete darkenss".to_string(), 0, W, true));
    items.push(objects.remove("egg beater").unwrap());
    world.push(Location::new(5, "Damp Tunnel".to_string(), "This rough earthen tunnel is dimly lit from above and sparkles ".to_string() +
                "with water seeping from its walls.", items, false, exits));
}

/// Test whether the hat was successfully placed in the starting Location
#[test]
fn object_is_in_location() {
    let mut w: Vec<Location> = Vec::new();
    let mut o: HashMap<String, Item> = HashMap::new();
    make_game_objects(&mut o);
    make_world(&mut w, &mut o);
    let hat: Item = Item::new("hat".to_string(), "a battered old fedora".to_string(), 2.0, None, Visible);
    assert_eq!(true, w[1].items.contains(&hat));
}

