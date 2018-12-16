#[derive(Debug)]
struct Foobar(i32);

fn double(num: Foobar) -> Foobar {
    Foobar(num.0 * 2)
}


fn main() {
    let x: Foobar = Foobar(1);
    let y: Foobar = double(x);
    println!("{}", y.0);
}
