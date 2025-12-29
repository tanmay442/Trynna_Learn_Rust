//need to do this bit to use the hashmap aka Dictionary
use std::collections::HashMap;


fn initialisingHashMap() -> HashMap<String, i32>{
    let mut things : HashMap<String,i32> = HashMap::new();
    
    //equivalent of push
    things.insert(String::from("keys"), 22);
    things.insert(String::from("lock"),2);
    things.insert(String::from("Debt"), -10);

    //{(keys:22), (lock:2)}
  return things
}

fn main() {
    let mut thingsHash = initialisingHashMap();
    println!("{:?}",thingsHash);

    let thingsHashLen = thingsHash.len();
    fro i in thingsHashLen{
        
    }

}
