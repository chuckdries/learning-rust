use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(self: &Self, fmt: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} ({} years old)", self.name, self.age)
    }
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Hello, {}!", alice);
}
