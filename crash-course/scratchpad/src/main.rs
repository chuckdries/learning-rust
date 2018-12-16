use std::env::args;
//use std::option::Option;

fn main() {
    for arg in args() {
        println!("{}", arg);
    }
}
