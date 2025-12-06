use std::{fmt::Pointer, panic::Location, process::Output};

fn main() {
    println!("Memory Management");
    //mutability()
    //stack_heaps()
    //ownership();
    //borrowing();


       
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
    //in rust the memory is managed by a set of rules called ownership that manages the memory for us
    //in rust a variable can have only one owner if the owner is removed then the variable also dies unless it is passed to another new owner
    //dangling pointer error are the eroors where the pointer in stack is pointing towards the heap element which no longer exist adn is deleted
    //double free eroor is when the heap data has been clared once by some pointer and then another variable try to clean it again which now might be in use by some diff application or not
    
    {let mut a =32;
    for i in 0..11{
        
        let b = a / 2; //b is a local variable with in this subscope of the for loop
        a=a * 2;
        
    };

    //println!("{}", b) // b is not in this scope
    print!("{}\n",a);
    };

    let s : String = String::from("Meow Meow MEow"); // _s is the owner of the variable Meow Meow Meow that is stored in the heap and has its pointer indicating towards it 
    println!("{} --initialised new ", s); // _s can be easily accessed
    let s_pointer = s.as_ptr(); //heap location of s
    
    let new_s = s; //now s is out of scope and does not exist all ownership of the variable is now with new s
    println!("{} ---changed ownership ",new_s); //new_s has now all the acces to the heap of s , note no new heap is formed no copy just the ownership changes from one vab to other
    let new_s_pointer = new_s.as_ptr(); //heap location of new_s

    if s_pointer == new_s_pointer{
        println!("they have the same pointer ")
    };      

    
    //s= s + &String::from("meow");
    //====output of above lien====
    //let new_s = s.clone(); //now s is out of scope and does not exist all ownership of the variable is now with new s

    let m : String = String::from("New Clone test"); // m being owner of the string newclonetest with some pointer
    let m_pointer = m.as_ptr();  //pointer or heap memory of m
    
    let clone_m = m.clone(); // cloning m means copy paste simply
    let clone_m_pointer = clone_m.as_ptr();

    println!("NOw trying cloning ");

    if m_pointer != clone_m_pointer{
        println!("Different pointers they got")
    };


    let meow : String = String::from("this data is in heap"); //initialised new variable
    println!("{}",meow);
    let meow_pointer=meow.as_ptr(); // pointer of meow_pointer

    //what happens when you pass a variable stored in heap as a arguement to another function
    //if it is something that is stored in a stack then it is cloned by default but if the variable is stored in heap
    //than the ownership of that variable is transfered from the variable to the function or some diff variaable in func

    fn take_ownership( mut new_owner_meow:String) -> String {
        println!("calling the person passing the variable as a arguement to change the ownership ");
    return new_owner_meow;
    }

    let a=take_ownership(meow); //now the owenership of variable is with a or the function , atm meow is out of scope 
    let a_pointer=a.as_ptr(); //checking pointer of a
    println!("{}",a);
    

    if a_pointer == meow_pointer{
        println!("yeah the transfer of ownership took place deifinetly")
    }

    //print!("{}",meow) this line would ccause eroor as meow is out of scope since the transfer of ownertship took place due to the function
    //to prevent meow goin out of scope again instead of passing it directly as arguemnt pass it cloning it 
    //take_ownership(meow.clone())

    let b=take_ownership(a.clone());
    if b.as_ptr()!=a_pointer{
        println!("yep cloning while passing as arguemtnt make them have deiif memory location without destroying the orignal variable and affecitng ownership basicall ycreating a new variable copy paste")
    }


    //ownership transfer and return back 
    // suppose you need to return the ownership to the initial variable how you do 
    //you make that variable mutable 
    //you pass it on the funtion and make it retrun as it is 
    //then you pipe your variable to the output of the function 
    //a example is below

    let mut qw:String = String::from("value");//ownership with qw
    qw=take_ownership(qw); //ownership passed to take_owner func but passed back again to qw
    print!("{}",qw)//teachnically this is just taking owenership from qw and making it out of scope and then creating a variable with same name and passing teh ownership to it 


}

fn borrowing(){
    //Borrowing is passing a view (pointer) to the value, allowing access without taking ownership.and using it as different varible
    //the refernce variable or the borrowed variable share the same pointer aka the same memeory location in heap
    //if the variable that is borrowing or making the reference dies the main variable and the prrimary ptr continue to exist reegardeless
    //The compiler prevents the owner from dying as long as a borrowed variable is still being used."
    
    //reference varaibale are basically just pointing towards the main primary one
    //borrwing is of two type mutable ,non mutable
    //any non mutable reference could be made to a variable
    
    //but only one mutable reference could be made
    //mutable refrence have the abiality to modify the heap memory aka the valur of variable 
    //this change also reflects in the owner or primary variable as they both are necassarily the same



    //MutableReferences
    println!(" \n\nWorking with mutable refrences now\n\n");

    //Initailised a primary variable
    let mut primary_variable_0 : String = String::from("The content or the value stored \nat some memory location in heap");
    let  primary_variable_0_pointer = primary_variable_0.as_ptr();
    
    //the "&" character indicate the refrence or borrowing status
    //mut shows it is borrowed mutably


    //fucntion borrowing /referencing the primary varaible scope mutably therfore not other ref atm can be formed  
    fn mut_borrow_primary_varible(input : &mut String) -> &String {
        let mut output=input;
        println!("Succesfully borrowed mutably");
        output.push_str("Meow");
        println!("Succesfully modified");
        return output;        
    }
    //once this function is completely executed the scope of this function does not exist
    //therefor the borrow or refrence that it has made is freed

    //the function is called mutable ref creaed , completed task and droppes 
    if primary_variable_0_pointer == mut_borrow_primary_varible(&mut primary_variable_0).as_ptr(){
            println!("Succesfully verified borrowing ");
            //since the function mut_borroe_primary_variable completed with in the if statemtnts conditional scope
            //the scope to make another reference or boorow is available now
            //therfore in the below statemtn we can again call it muatably with in the same function again            
            println!("{}",mut_borrow_primary_varible(&mut primary_variable_0)); 
            //once this statement is done therefore mutable reference scope is available again
        };


    // no other mutable reference made atm therefore one more can be made   
    let mut borrowed_0_primary_variable_0 = &mut primary_variable_0;

    //theese two line try to create more than one mutable refrences which is not possible therefore error
    //to see the error uncomment the next println
    //let mut borrowed_1_primary_variable_0 = &mut primary_variable_0; 
    //let mut borrowed_2_primary_variable_0 = &mut primary_variable_0; 

    //println!("{} {} {}",borrowed_0_primary_variable_0, borrowed_1_primary_variable_0,borrowed_2_primary_variable_0);






    //NonMutableReferences
    println!(" \n\nWorking woth non mutable refrences now\n\n");
    //Initialised another primary variable
    let mut primary_variable_1 = String::from("Another data");
    
    //any number of non mutable ref can be made but none of them can modify the primary variabl value , basicall read only
    let borrowed_0_primary_variable_1 =&primary_variable_1;
    let borrowed_1_primary_variable_1 =&primary_variable_1;

    fn non_mut_borrow_primary_varible(input: &String) -> &String{
        println!("--{}-- passed as a refrence non mutable", input);
        return input;

    }

    //3 non mutable refrences made at the same time no error can make any number of theese
    if borrowed_0_primary_variable_1 == borrowed_1_primary_variable_1 && borrowed_0_primary_variable_1 == non_mut_borrow_primary_varible(&primary_variable_1){
        println!("Multiple refrences made at the same time but since they all non mutable no problem ")
    }
    
}





fn string_methods_i_discovered_Randomly() {
    let mut string : String = String::from("Meow");
    string.push_str("add some more meow");
    println!("{}", string);
    string = string + &String::from("Meow");
    println!("{}",string);

}