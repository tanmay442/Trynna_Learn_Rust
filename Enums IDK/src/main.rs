use std::{ptr::dangling, sync::Arc};

#[derive(Debug)]

enum Direction{
    North,
    South,
    West,
    East
}

fn print_direction(direction:Direction)-> String{
    match direction {
        Direction::East => String::from("East"),
        Direction::North => String::from("North"),
        Direction::West => String::from("North"),
        Direction::South => String::from("North"),
    }
}


fn main() {
    println!("Hello, world!");
    let a=Direction::North;
    println!("{}",print_direction(a))
    
}

