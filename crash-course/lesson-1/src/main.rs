fn main() {
    let val: String = String::from("Hello, world!");
    printer(val.clone());
    printer(val);
}

fn printer(value: String) -> () {
    println!("value: {}", value);
}