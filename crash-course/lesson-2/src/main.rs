#[derive(Debug)]
struct Foobar(i32, i32);

fn main() {
    let x = Foobar(1, 2);
    foo(x);
}

fn foo(mut x: Foobar) {

    x.0 = 5; // changes the 0th value inside the product

    println!("{:?}", x);
}