// Vectors - resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[0] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(100);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vec length
    println!("Vector length: {}", numbers.len());

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        // dereference x
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);

    println!("-----------------------");
}
