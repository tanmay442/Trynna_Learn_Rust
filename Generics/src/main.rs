//while writing a function in srust you need to explicitly define the type of input and output to it 
//wehich can often feel reduntant so to fix that tust uses generics they are sort off a 
//general arguements that can act as a placeholder given that they are comparable

//https://doc.rust-lang.org/book/ch10-01-syntax.html


use std::any::type_name_of_val;

fn main() {
    let a = 4;
    let b = "meow";

    // This works!
    println!("Type of a: {}", type_identifier(&a));
    println!("Type of b: {}", type_identifier(&b));

    struct_example();
}

// The Generic Version: 
// T is a placeholder. Rust will create a version for i32 and a version for &str.
fn type_identifier<T>(variable: &T) -> &str {
    return type_name_of_val(variable);
}

// THIS WILL NOT COMPILE
// fn type_identifier_without_generic(variable: &???) -> &str {
//    return type_name_of_val(variable);
// }


struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn struct_example() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}