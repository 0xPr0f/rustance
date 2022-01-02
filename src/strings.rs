/* types of string --
primitive type and normal type
*/

pub fn run() {
    let mut hello = String::from("gett");
    println!("{}", hello);

    // Get the length of string
    println!("hello length is {}", hello.len());

    // push char to the end of a string
    hello.push('o');

    // push string to the end of a string
    hello.push_str(" is not for anybody");

    // Capacity in bytes
    println!("Hello capacity is {}", hello.capacity());

    // Check if string is empty
    println!("is it empty: {}", hello.is_empty());

    // if it contains substring "not"
    println!("contains 'not':  {}", hello.contains("not"));

    // Replace
    println!("Replace: {} ", hello.replace("not", "can be"));

    // Loop through string for whitespace
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('l');
    s.push('a');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
