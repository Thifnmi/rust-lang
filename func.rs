fn line() {
    println!("----------");
}

fn main() {
    // Functions Defined and invoke (call)
    line();
    line();
    line();

    // Functions Defined After Their Use
    f();
    fn f() {
        print!("Functions Defined After Their Use");
    }
}
