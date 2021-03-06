// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // Non-Primitive Vectors
    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. We will need to use a reference (&) to point to the resource

    println!("Values: {:?}", (arr1, arr2));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vec Values: {:?}", (&vec1, vec2));

    println!("-----------------------");
}
