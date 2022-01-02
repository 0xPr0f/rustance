// Fixed list where elements are the same data types

// importing std
use std::mem;
pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // ReAssign a value
    numbers[2] = 8;

    // Get array length
    println!("Array lemght: {}", numbers.len());

    // Get a position on an array
    println!("single value {}", numbers[0]);

    // Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice);

    // Print out the array
    println!("{:?}", numbers);
}
