// Variables in rust are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Haardik";

    // The following will lead to an error
    // cannot assign twice to immutable variable
    // let age = 21;
    // age = 22

    let mut age = 21;
    println!("My name is {} and I am {}", name, age);

    age = 22;
    println!("My name is {} and I am {}", name, age);

    // Define constants
    // need to explicitly define a type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Haardik", 21);
    println!("{} is {}", my_name, my_age);

    println!("-----------------------");
}
