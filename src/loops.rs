pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        // Break manually
        if count >= 5 {
            break;
        }
    }

    count = 1;

    // While loop (fizzbuzz)
    while count <= 20 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // For Range loop
    for x in 0..10 {
        println!("x: {}", x);
    }

    println!("-----------------------");
}
