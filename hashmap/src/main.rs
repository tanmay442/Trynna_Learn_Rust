use core::panic::PanicMessage;
//need to do this bit to use the hashmap aka Dictionary
use std::{collections::{HashMap, btree_map::Values}, ops::Shl};


fn initialisingHashMap() -> HashMap<String, i32>{
    let mut things : HashMap<String,i32> = HashMap::new();
    
    //equivalent of push
    things.insert(String::from("keys"), 22);
    things.insert(String::from("lock"),2);
    things.insert(String::from("Debt"), -10);
    //things.insert(String::from("meow"),66);
    
    //{(keys:22), (lock:2)}
  return things
}


fn main() {
    let mut thingsHash = initialisingHashMap();
    println!("{:?}",thingsHash);

    //getting a  element out of hashmap returns a option<valueType> either pattern match or unwrap
    println!("{}",thingsHash.get("keys").unwrap());
    let hashElement = thingsHash.get("meow");
    match hashElement {
      Some(Values) => println!("{}",Values),
      None => println!("None value in scope")
        
    }

    //lenght of a hashmap
    let a=thingsHash.len();
    println!("the size of the hashmap is {}", a);
   
    //indexing or something appareantly indexing hashmpa results you in a touple
    for i in thingsHash{
        println!("this is the key {} || this is the value {} ",i.0, i.1);
    }
    
  someMixedTupleVectorAndHashMapProblem();
}


//someMixedTupleVectorAndHashMapProblem

fn someMixedTupleVectorAndHashMapProblem() {

  let pVec = vec![
    ("Alice", 90),
    ("Bob", 85),
    ("Alice", 95),
    ("Charlie", 80),
    ("Bob", 70)
  ];

  let mut sHash = HashMap::new();


  for i in pVec{
    //print!("{:?}",i);
    if  !sHash.contains_key(i.0){
      sHash.insert(i.0, vec![i.1]);
    }

    else{
      let mut firsttrackedElement: Vec<i32> = sHash.get(i.0).unwrap().clone();
      let mut trackedElement = Vec::new();
      trackedElement.append(&mut firsttrackedElement);
      trackedElement.push(i.1);

      sHash.insert(i.0, trackedElement);

    }
  
  }

println!("{:?}",sHash)

  

}