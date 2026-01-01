//A String type which is coded into tust std library rather than coded into core language is a 
//growable mutable owned utf8 encoded string type

//Slices let u refrence a contiguous sequence of elements in a collection rather rather than whole collection
//it is a type of reference, so it does not have ownership



fn string_soomething() {
    let mut word = String::from("Meow");

    //pushing a word in a string  essentially a string is a type of collection so push works in similiar way as in vecotrs
    word.push_str(" Meow");
    println!("{}", word);

    //how to replace something in a String
    word.replace_range(4..word.len()," rawr");
    println!("string after operting replacement {}", word)

}


fn main(){
    string_soomething();
    let something = String::from("meow mewowmewomew meowmeomeow mweowe meowe emwoe ");
    println!("{}",returnFirstWord(&something));
    println!("{:?}",word(&something));
    println!("{}",word_efficient(&something))


    //String literal 
    //they are basically hardcoded into the binary of the program 
    //to create them is simple just create a string like you would do in any language no rust bs 
    //but htis also mean no rust features or i should say any feature 
    //only good for static fields , that are never gonna be edited
    //the type is still &str 
    //but hte pointer points ot the literall biunary file instead of any place in memory

    let  stringliteral = "meow";
    
    
}


// this is inefficint
//creates two variable 
//could use  just a stiring slice

fn returnFirstWord(something : &String) -> String {
    let mut ans = String::from("");
    let somethingLen = something.len();

    for i in 0..somethingLen{
        let letter = something.chars().nth(i).unwrap();
        if letter.to_string().as_str() == " " {
            break
        }
        ans.push_str(&letter.to_string());
    }
    //let a = &something[0..4];
    //println!("{}",a);
    return ans

}


//using string slice is better it prevents memory managemnet error
fn word (something : &String) -> &str{
    let lenSomething = something.len();
    let mut end : usize = 0;
    for i in 0..lenSomething{
        let letter = something.chars().nth(i).unwrap();
        if letter.to_string().as_str() == " "{
            end = end + i;
            break
        }
    }
    return &something[0..end]
    
}

fn word_efficient(something: &str) -> &str {
    something.split_whitespace().next().unwrap_or("")
}


