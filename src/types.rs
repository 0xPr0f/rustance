/*
Primitive types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Float: f32, f64
Boolean (bool)
Characters (char)
tuples
arrays
*/

pub fn run() {
    // Defualt is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit types
    let z: i64 = 40909098;

    let char = '😂';

    // find max size
    println!("Max i32 : {}", std::i32::MAX);
    println!("Max i64 : {}", std::i64::MAX);

    // Boolean
    let isactive: bool = true;
    println!("{:?}", (isactive, x, y, z));

    //Get bool from expression
    let is_greater = 10 < 5;
    println!("{}", is_greater);

    println!("{}", char)
}
