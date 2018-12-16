use std::env::args;

fn main() {
    for arg in args().skip(1) {
        println!("{:?}", arg.parse::<u32>());
    }
}
