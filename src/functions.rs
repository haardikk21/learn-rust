pub fn run() {
    greeting("Hello", "Haardik");

    // Function values to vars
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    let dif = sub(10, 5);
    println!("Diff: {}", dif);

    // Closures (pipes)
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure Sum: {}", add_nums(3, 4));

    println!("-----------------------");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // Can return by omitting semicolon
    n1 + n2
}

fn sub(n1: i32, n2: i32) -> i32 {
    // Can return by including return statement with semicolon
    return n1 - n2;
}
