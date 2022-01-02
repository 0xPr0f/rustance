// Variables holds primitive data or reference to data
// Rust is blocked-scoped
// Variables are immutable by default

pub fn run() {
    // Normal var assignment
    let name = "Brad";
    // making vars mutable
    let mut age = 37;
    println!("My name is {} and i am {}", name, age);
    age = 38;
    println!("My name is {} and i am {}", name, age);

    // Define contant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at ones
    let (my_age, my_name) = (37, "prof");
    println!("{} is {} years old", my_name, my_age);
}
