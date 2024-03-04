fn main() {
    signed_integer_number();
    unsigned_integer_number();
    float_number();
    suffixes_of_number();
}

fn signed_integer_number() {
    let x: i8 = -127;
    print!("signed integer number {}.", x);
}

fn unsigned_integer_number() {
    // let x: u8 = -127; // error
    let x: u8 = 255;
    print!("\nUnsigned integer number {}.", x);
}

fn float_number() {
    let x = 3.141_592_653_589_793_238_462_643_3;
    let y: f32 = 3.141_592_653_589_793_238_462_643_3;
    let z: f64 = 3.141_592_653_589_793_238_462_643_3;
    print!("\nfloating-point number default {}, f32 {}, f64 {}.", x, y, z);
}

fn suffixes_of_number() {
    let _a: i16 = -150;
    let _b = -150 as i16;
    let _c = -150 + _b - _b;
    let _d = -150i16;
    let _e = -150_i16;
    print!("\nsuffixes of number _a {}, _b {}, _c {}, _d {}, _e {}.", _a, _b, _c, _d, _e);
}
