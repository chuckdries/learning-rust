use std::env::args;
use std::option::Option;

fn main() {
    let mut args = args();
    loop {
        let argument = args.next();
        let value = match argument {
            Option::None => break,
            Option::Some(t) => t,
        };
        println!("{:?}",value);
    }
}
