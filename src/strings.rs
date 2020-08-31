/*
Primitive str = Immutable fixed length string somewhere in memory
'String' type = Growable, heap allocated data structure
*/

pub fn run() {
    let primitive_hello = "Hello";
    let mut hello = String::from("Hello");

    // Get length
    println!("Primitive len: {}", primitive_hello.len());
    println!("Hello len: {}", hello.len());

    // primitive_hello cannot be pushed to

    // Push char
    hello.push(' ');

    // Push string
    hello.push_str("World");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains `World` {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    // Nothing happens if pass, panic if fail
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    // assert_eq!(3, s.len());

    println!("{}", s);

    println!("-----------------------");
}
