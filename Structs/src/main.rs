//Structs 
fn main() {
    println!("Hello, world!");

    //adding mut allow u to modify it
    let mut pou = user{
        name: String::from("POU"),
        age: 19
    };
    println!("{} is {} year old",pou.name, pou.age);
    
    let nou = user{
        name: String::from("nou"),
        age : 17
    };
    println!("{}, is {} years old",nou.name, nou.age);

    //if not for mut pou the line below reuslt in error
    pou.name=String::from("Meow");
    println!("{} is {} year old",pou.name, pou.age);

    let rect= area{
        width:5,
        length:10
    };
    println!("{}",rect.aarreeaa());
}


struct user{
    name: String,
    age: i8,
}


//implemtng Structs


// all the constant variable defined here , they need to be defined at the compile time
struct area{
    width: u32,
    length: u32
}

//all the fucntional computing transformation you need to do with a struc varibales needed to be done in a different 
// section called impl which is called or written like this

//&self is passed as arguement :) yk why
impl area{
    fn aarreeaa(&self)-> u32{
        return self.width * self.length
    }
}

