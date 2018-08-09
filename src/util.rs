/* Neil Babson
*  August 2018
*/

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Direction {
    N, S, E, W, NE, SE, NW, SW, }

#[derive(Debug)]
pub struct Location {
    pub index: usize,
    pub name: String,     
    pub description: String,
    pub items: Vec<Item>,                 
    pub visited: bool,
    pub exits: Vec<Exit>,
}    

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

#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: f32,
}    

impl Item {
    pub fn new(n: String, d: String, w: f32) -> Item {
        Item { name: n, description: d, weight: w}
    }
}

#[derive(Debug)]
pub struct Exit {
    pub description: String,
    pub goes_to: usize,    
    pub direction: Direction,
    pub locked: bool,
    //goes_to: &'a Location,
}                 

impl Exit {
    pub fn new(d: String, g: usize, dir: Direction, lock: bool) -> Exit {
        Exit { description: d, goes_to: g, direction: dir, locked: lock}
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub inventory: Vec<Item>,
    pub score: i32,
    pub turns: i32,
}

impl Player {
    pub fn new(n: String, i: Vec<Item>) -> Player {
        Player { name: n, inventory: i, score: 0, turns:0 }
    }
}    


