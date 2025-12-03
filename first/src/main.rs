fn main() {
    println!("Hello, world!");
    let mut x: f32 = 3.14;
    let mut y: f32=8.00;
    println!("the number should be {}",x);
    for _i in 0..5{
        x = x + 100.0 * y;
        if y % 2.00 == 0.0{
            y = y+1.00
        }
        println!("{}",x);
    }
    //meow(8);
    //let a : String = String::from("Meow meow meow");
    //string_indexing(a);
    //println!("{}",first_word(a));

  }

fn meow(a:u8) {
    let mut meow : String  = String::from("Meow");
    //print!("{}", meow)
    for i in 0..a{
        print!("{}",meow);
        for _j in 0..i{
            print!("meow");
            meow = meow + &String::from("Meow");

        };
        print!("\n");

    };

    }

fn string_indexing(a:String) {
    let b:usize =a.chars().count();
    for i in 0..b{
        let character = a.chars().nth(i);
        match character{
            Some(character) => print!("{} : {} \n", character, i),
            None => print!("None"),
        };
        

    }


}

fn first_word(line : String) -> String {
    let line_size = line.chars().count();
    let mut ans : String = String::new();
    for i in 0..line_size{
        let letter = line.chars().nth(i);
        match letter{
            Some(letter) => {
                ans=ans + &String::from(letter);
                if letter.to_string().as_str()==" "{
                    break;
                }; 
            }
            None => print!("Idk man"),
            
        };
    };
    return ans;
}


