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

