fn f(x: i32) {
    if x <= 0 {
        return;
    }
    if x == 4 {
        return ();
    }
    if x == 7 {
        return {};
    }
    print!("{}", x);
}

fn f1() -> i32 {
    3
}
fn main() {
    f(7);
    f1();
    let a = f1();
    println!("{a}")
}
