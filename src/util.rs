/* Neil Babson
*  August 2018
*/

use std::collections::HashSet;

pub const MAX_WEIGHT: f32 = 25.0;

/// Enum of the permissible movement directions
#[derive(Debug, PartialEq)]
pub enum Direction {
    N, S, E, W, NE, SE, NW, SW, U, D,
}

/// Visibility possabilities for an object
/// A Hidden object cannot be seen via look are taken
/// A Seen object is not visible to the player but
/// they know it is there so it can be taken
#[derive(Debug, PartialEq)]
pub enum Visibility {
    Visible, Hidden, Seen}

/// Game location stuct containing a vector of
/// Item objects and a vector of Exit objects    
#[derive(Debug)]
pub struct Location {
    pub index: usize,
    pub name: String,     
    pub description: String,
    pub items: Vec<Item>,                 
    pub visited: bool,
    pub exits: Vec<Exit>,
}    

/// Implement a new Location
impl Location {
    pub fn new(ind: usize, n: String, d: String, it: Vec<Item>, v: bool, e: Vec<Exit>) -> Location {
        Location { index: ind,
                   name: n,
                   description: d,
                   items: it,
                   visited: v,
                   exits: e, }
    }
}

/// An Item has weight, the option of containing another object, and its current visibility status
#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub contains: Option<String>,
    pub visibility: Visibility,
}    

/// Implement a new Item
impl Item {
    pub fn new(n: String, d: String, w: f32, c: Option<String>, v: Visibility) -> Item {
        Item { name: n, description: d, weight: w, contains: c, visibility: v,}
    }
}

/// An Exit has the index of the Location it leads to as well as the direction it leads in
/// and its locked/unlocked status
#[derive(Debug)]
pub struct Exit {
    pub description: String,
    pub goes_to: usize,    
    pub direction: Direction,
    pub locked: bool,
}                 

/// Implement a new Exit
impl Exit {
    pub fn new(d: String, g: usize, dir: Direction, lock: bool) -> Exit {
        Exit { description: d, goes_to: g, direction: dir, locked: lock}
    }
}

/// A player has their inventory as vector of Items, current score, game turns played, and weight
/// carried. A vector of boolean flags represent whether certain game events have occured yet.
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub inventory: Vec<Item>,
    pub score: i32,
    pub turns: i32,
    pub weight: f32,
    // Event 0: find grate under statue
    pub events: Vec<bool>,
}

/// Implement a new Player
impl Player {
    pub fn new(n: String, i: Vec<Item>, e: Vec<bool>) -> Player {
        // Player starts with crystal ball with weight 6.5
        Player { name: n, inventory: i, score: 0, turns:0, weight:6.5, events: e }
    }
}    


