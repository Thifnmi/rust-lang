fn main() {
    // invalid
    // let two = 2.;
    // print_double(17.2);
    // fn print_double(x: f64) {
    //     print!("{}", x * two);
    // }

    // valid
    const TWO: f64 = 2.;
    fn print_double(x: f64) {
        print!("{}", x * TWO);
    }
    print_double(17.2);
    // or
    // static TWO: f64 = 2.;
    // fn print_double(x: f64) {
    //     print!("{}", x * TWO);
    // }
    // print_double(17.2);
}
