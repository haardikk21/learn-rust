// Command Line args

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let status = "100%";

    // cargo run hello haardik
    println!("Args: {:?}", args);

    println!("Command: {}", command);

    if command == "hello" {
        let name = args[2].clone();
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }

    println!("-----------------------");
}
