#[derive(Debug)]
struct Foobar(i32);

impl Clone for Foobar {
    fn clone(&self) -> Foobar {
        Foobar(self.0)
    }
}

impl Copy for Foobar {}

fn uses_foobar(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x = Foobar(1);
    uses_foobar(x);
    uses_foobar(x);
}