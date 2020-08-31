pub fn run() {
    let age = 21;
    let check_id = false;
    let knows_person_of_age = true;

    // If/E;se
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

    println!("-----------------------");
}
