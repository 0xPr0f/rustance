pub fn run() {
    // Print to console
    println!("print from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is a {}", "tunde", "boy");
    println!("I love rust cause it is {}", "awesome");

    // Positional Arguements
    // use {} with index to asign values anywhere
    println!(
        "{0} is a {1} that deals with {2} progam, which makes {0} {3}.",
        "Brad", "programmer", "rust", "awesome"
    );

    // Named Activity
    // use {} with identifiers/names
    println!(
        "{name} likes to play to {game}.",
        name = "prof",
        game = "warzone"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octa: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, "is", true));

    // Basic Maths
    println!("10 + 10 = {}", 10+10)
}
