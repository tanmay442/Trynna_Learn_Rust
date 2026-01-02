//traits are basically abastract classes 
//in a tratit you  primiarily define some fn that you can use with a structs

//if u try to call a function on a struct which is not defind in the imported trait the compileer wont compile
//traits are basically for abstarction of fn in structs 


//ok so akthually i can implemtnt a triat on anyhting 
//literarlly i can apply it to default funcs like String and anything for say

pub trait something{
    fn do_something(&self)-> Option<f32>{
        Some(0.0)
    }
}


struct points{
    x1 : f32,
    x2 : f32,
    operator : String
}

//trait implemented gotta have the trait defined funcs or the default be used
//here by defining the same func we a re overriding the trait somewhat like inline css canceling classes
impl something for points{
    fn do_something(&self)-> Option<f32>{
        if self.operator == "add"{
            Some(self.x1 + self.x2)
        }
        else {
            println!("lmao i only know addition here are your points tho ({},{})",self.x1,self.x2);
            None
        }
    }
}

struct sot{
    ab:String
}

impl something for sot{}

//implementing soemthing on Default STrign func
impl something for String {}


//Traits could also be used as parameter to the function by restricting the scope of the variable 
//if there is a trait passed in as the arguement to thre function the acceptable input to the function
//are the same as the acceptable inputs to the traits

pub fn did_soemthing(stuff: &impl something)-> Option<f32>{
    stuff.do_something()
}

fn main() {
    let first_set = points{x1:1.1,x2:2.1,operator:String::from("add")};
    println!("{}",first_set.do_something().unwrap());

    let second_set = points{x1:2.0,x2:3.0,operator:String::from("multiply")};
    println!("{}",second_set.do_something().unwrap_or(0.0));


    //calling the trait func on a struct with no local fun imp
    let a = sot{ab:String::from("meow")};
    println!("should return {}", a.do_something().unwrap() );

    //calling the Trait on literal String
    println!("calling the trait on String def func here is the output : {}", String::from("meow").do_something().unwrap());

    println!("calling the trait passed down in a func as a parameter \nhere is the output {} ",did_soemthing(&a).unwrap())
}
