//an iterator is responsible for iterating over a sequence of some kind over each item 
//and determining when the sequence is finished
//in rust iterators are lazy meaninig they have no effect until they are called in
//under the hood every loop over some collection uses an iterator


fn main() {

    let mut vector = Vec::new();
    vector.push("meow");
    vector.push("meowmeow");
    vector.push("meowmeowmeow");
    

    
    //this is a iterator
    //here thisi is a example of the immutable iterator 
    //it doesn't consumes the value just boorows the value basically creting a pointer to memeory location
    let vectorIter = vector.iter();
   
    for valVector in vectorIter{
        println!("{}",valVector)
    } 


    //new varible
    let mut vec = vec![1,2,3,4,5,6,7,8];

    //creating a mutable iterator
    let mut vecIter = vec.iter_mut();

    for i in vecIter{
        *i = *i *2  ;
    }

    //reflects the change in values
    println!("{:?}", vec);

    let mut vec : Vec<i16> = vec![2,3,4,5,6,5,3,3,2,2];
    println!("{:?}", whileLoops(vec));

    let mut newvec : Vec<i16> = vec![1,2,23];
    intoIter(newvec);

    Adapters();
}

//into_iter
//takes the ownership of the variable , rendering the orignal varible out of scope
//slight performance and memory opt
fn intoIter(a:Vec<i16>){
    let  aIntoIter = a.into_iter();
    for i in aIntoIter{
        println!("{}",i);
    }
    //this value is boorowed will result in borrow error
    //println!("{:?}",a);
}



// while loop syntax
fn whileLoops (mut a:Vec<i16>) -> Vec<i16> {
    let mut iterator = a.iter_mut();
    while let Some(values)= iterator.next(){
        *values= values.pow(5) ;        
    }
     return a
}


// === Adapters ====

//Consuming Adapters (e.g., .sum()): These take ownership of the iterator and exhaust it (run it until the end) to produce a value.
//Iterator Adapters (e.g., .map()): These take ownership of the iterator and wrap it into a new iterator. They are "lazy"—they don't actually do anything until you call a consuming adapter.


fn Adapters(){
    let v = vec![1,2,3,4];
    
    let vIter = v.iter();
    
    //.sum() is a consumer adapter
    let sumV : i16 = vIter.sum();
    println!("{}",sumV);
    
    //this would give me a error since the iterator is out of scope ##note the vector is not out of scope only iterator
    //println!("{:?}",vIter);

    //this is a iterator adapter
    let v1Iter = v.iter();

    //the v1iter is used up here
    let v2Ier = v1Iter.map(|x| x+1 );
    println!("{:?}",v2Ier); 

    //the iterator v2ier is used up here
    let result: Vec<_> = v2Ier.collect();
    println!("{:?}",result);


    //similiarly is the filter func
    let v3Iter = v.iter().filter(|x| *x%2 == 0);
    let result2: Vec<_> =v3Iter.collect();
    println!("{:?}",result2)


}