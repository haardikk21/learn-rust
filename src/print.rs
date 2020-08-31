pub fn run() {
    // Print to console
    println!("Hello from print.rs");

    // Formatting
    println!("{} is from {}", "Haardik", "India");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Haardik", "India", "code"
    );

    // Named arguments]
    println!(
        "{name} likes to play {sport}",
        name = "Haardik",
        sport = "badminton"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Debug trait
    println!("{:?}", (12, true, "hello"));

    // Math
    println!("10 + 10 = {}", 10 + 10);

    println!("-----------------------");
}
