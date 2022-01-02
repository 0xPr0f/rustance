// Tuples max is 12 element
//Tuples group diffent data types

pub fn run() {
    let person: (&str, &str, i8) = ("prof", "mars", 15);

    println!(
        "{} is from {} and is {} years old",
        person.0, person.1, person.2
    )
}
