use std::env::args;
//use std::option::Option;

fn main() {
    let mut args = args();
    while let Some(argument) = args.next() {
        println!("{:?}",argument);
    }
}
