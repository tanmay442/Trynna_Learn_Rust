//Error Handling and null handling
//match is teh goat :goatemoji

use std::fs;

fn main() {

    println!("Hello, world!");

    let res= fs::read_to_string("src/example.txt"); //would result in error if not for error handled
       
    match res{
        Ok(content) => {println!("{}",content);},
        
        Err(err) => {println!("Errror : {}",err)},
    }

    let a: String = String::from("Meow");
    let len_a = a.len();

    //the below forLoop goes out of range overflow errror shoul rise lets handle that
    for i in 0..len_a+2{
        let numberAtIndex = a.chars().nth(i);
        match numberAtIndex{
            Some(content) => {
                println!("{}: {:?}",i,content);
            },
            None => {
                println!("MEow Meow happened to your range fix that bozo")
            },
        }
        //println!("{}: {:?}",i,a.chars().nth(i).unwrap());
    };  
}


//unwrap make you retun the char value from option
//a.chars().nth(i) returns a option
//a.chars().nth(i).unwrap() returns the char directly