fn print_double(mut x: f64) {
    x *= 2.0;
    println!("value in func print_double {}", x);
}
fn main() {
    let x = 4.0;
    print_double(x);
    print!("{}", x);
}
