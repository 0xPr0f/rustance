use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // ReAssign a value
    numbers[2] = 8;

    // Add on to vector
    numbers.push(5);

    //  remove last value
    numbers.pop();

    // Get Vector length
    println!("Vector lemght: {}", numbers.len());

    // Get a position on a vector
    println!("single value {}", numbers[0]);

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice);

    // Print out the array
    println!("{:?}", numbers);
}
