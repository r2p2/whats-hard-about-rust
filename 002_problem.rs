struct Foo {
    foo_a: i32,
    foo_a2: i32,
}

impl Foo {
    pub fn new(a: i32) -> Foo {
        Foo {
            foo_a: a,
            foo_a2: foo_a, // nope
        }
    }
}

fn main() {
    let mut f = Foo::new(2);
}
