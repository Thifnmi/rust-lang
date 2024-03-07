fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { Err(format!("Divide by zero")) } else { Ok(numerator / denominator) }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}

fn main() {
    print!("{:?}, {:?}", divide(8.0, 2.0), divide(8.0, 0.0));
    show_divide(8.0, 2.0);
    show_divide(8.0, 0.0);
    let r1 = divide(8.0, 2.0);
    let r2 = divide(8.0, 0.0);
    println!("{} {}", r1.is_ok(), r2.is_ok());
    println!("{} {}", r1.is_err(), r2.is_err());
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
    let mut v = vec![11, 22, 33]; for _ in 0..v.len() {
        print!("{}, ", v.pop().unwrap())
    }
}
