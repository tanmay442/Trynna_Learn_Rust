fn main() {
    println!("Initialising the Vector");

    //===initialiisngVector ======
    let  mut my_vec = Vec::new();
    my_vec.push(1);
    my_vec.push(2);
    println!("{:?}", my_vec);
   
    //=== pushingSomeValue ===========
    let mut j =0;
    while my_vec.len()<50 {
        my_vec.push(j);
        j = j+1;

    }

    for i in my_vec{
        if i % 2 == 0{
            println!("{:?}",i)
        }

    }
    //shortHand for creating a Vector
    let newVector = vec!["Raju", "Anand", "Aaalu", "Meow"];  
    vectorOperation(newVector);

}

fn vectorOperation(someVector:Vec<&str>){
    for vectorValue in someVector{
        if (vectorValue.chars().nth(0)).unwrap_or('B') == 'A' {
            println!("{:?}", vectorValue)
        }
    }     
}

