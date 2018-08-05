

#[derive(Debug)]
pub struct Location {
    pub index: usize,
    pub name: String,     
    pub description: String,
    pub items: Vec<Item>,                 
    pub visited: bool,
    pub exits: Vec<Exit>,
}    

#[derive(Debug)]
pub struct Item {
    pub description: String,
    pub weight: f32,
}    

impl Item {
    pub fn new(d: String, w: f32) -> Item {
        Item { description: d, weight: w}
    }
}

#[derive(Debug)]
pub struct Exit {
    description: String,
    goes_to: usize,    
    //goes_to: &'a Location,
}                 


pub fn test() {
    println!("test");
}
