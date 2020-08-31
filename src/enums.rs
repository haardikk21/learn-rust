enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on movement
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let m1 = Movement::Left;
    let m2 = Movement::Up;
    let m3 = Movement::Right;
    let m4 = Movement::Down;

    move_avatar(m1);
    move_avatar(m2);
    move_avatar(m3);
    move_avatar(m4);
    println!("-----------------------");
}
