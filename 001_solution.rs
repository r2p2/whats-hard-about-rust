struct Foo {
    a: Vec<i32>,
    result: Vec<i32>,
}

impl Foo {
    pub fn new() -> Foo {
        Foo {
            a: vec![5; 10],
            result: vec![0; 10],
        }
    }

    pub fn calc(&mut self) {
        for (i, item) in self.result.iter_mut().enumerate() {
            *item = Self::add(&self.a, i);
        }
    }

    fn add(a: &Vec<i32>, i: usize) -> i32 {
        a[i] * i as i32
    }
}

fn main() {
    let mut f = Foo::new();
    f.calc()
}

