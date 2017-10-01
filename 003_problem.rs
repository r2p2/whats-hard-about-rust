fn f2(i: &mut i32) {
    *i *= 2
}

fn f1(i: &mut i32) {
    f2(i)             // &mut forbidden
}

fn main() {
    let mut i = 10;
    f1(&mut i);       // &mut required
    println!("{}", i);
}
