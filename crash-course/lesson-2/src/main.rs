#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

fn uses_foobar(foobar: &Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x: Foobar = Foobar(1);
    let y: &Foobar = &x;
    println!("Before uses_foobar");
    uses_foobar(&x);
    uses_foobar(y); // why does this call work with and without the &?
    uses_foobar(&y); // why does this still work after y was consumed?
    println!("After uses_foobar");
}