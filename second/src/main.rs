fn main() {
    println!("Memory Management");
    //mutability()
    //stack_heaps()
    //string_methods_i_discovered_Randomly()
}


fn mutability() {
    println!("By default every variable in rust is immutable");
    let mut x: String = String::from("Meow Meow"); //the keyword mut is added to make the String mutable means to be able to perform the string operation or any sort of change to it giving us acces to chang the memory heap
    x.push_str(&String::from(" Meow"));
    println!("{}",x);
    
}

fn stack_heaps() {
    //stack stores all the predictive data like i32 and all which is in a dedicated allocated part of the ram
    //heap on the other hand is used to store the unpredeictive data like a mutable string whoose lenght or size is 
    //not predicatable and is stored in a shared memory along with the OS and kernel , the address to the location where
    //the data in heap is stored in the stack so that the application can point towardst that memory
    //heap being dedicated allocated ram is faster than local memory of heap
    //heap is used to store the dat awhich can grow at runtime, such as strings and vector
    //stack is used to store int , bools and fixed arrays also the pointers to heap elements
    //stacl is for predicatable compile time data
    

    let a : i8 = 120; //stored in heap during CompileTime
    
    let mut b : String = String::from("Meow"); //the string meow is stored in a heap with the pointer of it being in a stack , during CompileTime


    println!("capacity {}, length {} , pointer {:p}",b.capacity(),b.len(),b.as_ptr()); //the result of this line of code is stored in stack , wehre the pointer points twoard the memory address of heap

    b = b + &String::from("Meow"); //changing the variable at the RunTime modifying the heap and pointer if required by OS
    
    println!("capacity {}, length {} , pointer {:p}", b.capacity(), b.len(), b.as_ptr()); //the data after being modified reflects chages 

}

fn ownership() {
    
}







fn string_methods_i_discovered_Randomly() {
    let mut string : String = String::from("Meow");
    string.push_str("add some more meow");
    println!("{}", string);
    string = string + &String::from("Meow");
    println!("{}",string);

}