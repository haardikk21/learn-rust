/*
Primitive Types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, u256, i256
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays
*/

// Rust is a statically typed language, but the compiler can usually infer what type we want to use based on the value and how we use it

use std;

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicit type
    let z: i64 = 32423234235345234;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 5 > 3;

    // Char
    let a1 = 'a';
    let smiley_face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, smiley_face));

    println!("-----------------------");
}
