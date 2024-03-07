fn main() {
    // square_root(45.2f32);
    println!("{}", f('a', 37, 41));

    let a: i16 = f1::<i16>('a', 37, 41);
    let b: f64 = f1::<f64>('b', 37.2, 41.1);
    println!("{} {}", a, b);

    let a1: i16 = f1('a', 37, 41);
    let b1: f64 = f1('b', 37.2, 41.1);
    println!("{} {}", a1, b1);

    let _a = f2(12u8, 13u8);
    let _b = f2(12i64, 13i64);
    // let _c = f2(12i16, 13u16);
    // let _d: i32 = f2(12i16, 13i16);

    println!("{:?}", f3('a', true));
    println!("{:?}", f3(12.56, "Hello"));
    println!("{:?}", f3((3, 'a'), [5, 6, 7]));
}

// fn square_root(x: f32) -> f32 {}

fn f(ch: char, num1: i16, num2: i16) -> i16 {
    if ch == 'a' { num1 } else { num2 }
}

fn f1<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 } else { num2 }
}

fn f2<T>(a: T, _b: T) -> T { a }

fn f3<Param1, Param2>(a: Param1, b: Param2) -> (Param2, Param1) {(b, a)}
