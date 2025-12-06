# My Rust Learning Log

> [!Rule]
> **Rule:** Turn off Copilot while learning. It helps get a grasp of the syntax faster and better.

Just a place to dump my learnings and experiments.

## Memory Management
This stuff is deep.

- **Stack vs Heap:** Stack is for predictive data (i32, bool), Heap is for unpredictable data (Strings).
- **Ownership:** "in rust a variable can have only one owner if the owner is removed then the variable also dies unless it is passed to another new owner"
- **Borrowing:** Passing a view (pointer) without taking ownership.
    - "mutable refrence have the abiality to modify the heap memory"
    - "The compiler prevents the owner from dying as long as a borrowed variable is still being used."

```rust
// fucntion borrowing /referencing the primary varaibale scope mutably
fn mut_borrow_primary_varible(input : &mut String) -> &String {
    let mut output=input;
    println!("Succesfully borrowed mutably");
    output.push_str("Meow"); // modifying the heap
    return output;        
}
```

## Null Value & Error Handling
**match is teh goat :goatemoji**

- `unwrap` makes you return the value from an Option directly.
- `Option` returns `Some(content)` or `None`.

```rust
let res= fs::read_to_string("src/example.txt"); 
   
match res{
    Ok(content) => {println!("{}",content);},
    Err(err) => {println!("Errror : {}",err)},
}
```

## Structs
- Adding `mut` allows you to modify the struct fields.
- `impl` blocks are where the functional computing happens.
- `&self` is passed as argument :) yk why.

```rust
struct user{
    name: String,
    age: i8,
}

// if not for mut pou the line below reuslt in error
pou.name=String::from("Meow");
```

## Enums
Just standard enums, but pattern matching makes them powerful.

```rust
enum Direction{
    North,
    South,
    West,
    East
}

fn print_direction(direction:Direction)-> String{
    match direction {
        Direction::East => String::from("East"),
        // ...
    }
}
```

## Syntax
- By default every variable is immutable.
- `let mut x` to make it mutable.

```rust
let mut x: f32 = 3.14;
for _i in 0..5{
    x = x + 100.0 * y;
}
```
