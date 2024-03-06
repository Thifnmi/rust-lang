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

    // Passing args
    let x = 4.0;
    print_double(x);
    print!("{}", x);

    // Passing Arguments to a Function
    print_sum(3.0, 5.0);
    print_sum(3.2, 5.1);

    // Returning a Value from a Function
    println!("{}", double(17.3));

    // Returning Several Values
    println!("{:?}", several_values(12, 9));

    // Functions Shadowing Other Functions
    println!("{} ", match f1() {
        E::E1 => 1,
        _ => -1,
    });
    println!("{} ", f2().a);
    println!("{} ", f3().0);
    println!("{} ", f4()[0]);
    println!("{} ", f5()[0]);

    // Passing args reference by reference
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    print!("{:?}", arr);


    // mutable reference
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a;
    println!("{} {}", *p, p);
    *p += 1;
    println!("{} {}", *p, p);
    p = &mut b;
    println!("{} {}", *p, p);
    *p += 1;
    println!("{} {}", *p, p);

    // Using reference
    let a = 15;
    let ref_a = &a;
    print!("{} {} {} {}", a, *ref_a, ref_a, &ref_a);
    let a = &&&7;
    print!("{} {} {} {}", ***a, **a, *a, a);
}

fn print_double(mut x: f64) {
    x *= 2.0;
    println!("value in func print_double {}", x);
}

fn print_sum(addend1: f64, addend2: f64) {
    println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
}

fn double(x: f64) -> f64 {
    x * 2.0
}

// Returning Several Values
fn several_values(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// Functions Shadowing Other Functions
enum E {
    E1,
    E2,
}
struct S {
    a: i32,
    b: bool,
}
struct TS(f64, char);
fn f1() -> E {
    E::E2
}
fn f2() -> S {
    S { a: 49, b: true }
}
fn f3() -> TS {
    TS(4.7, 'w')
}
fn f4() -> [i16; 4] {
    [7, -2, 0, 19]
}
fn f5() -> Vec<i64> {
    vec![12000]
}

// Change a Variable of the Caller
fn nomal() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    for i in 0..10 {
        if arr[i] < 0 {
            arr[i] *= 2;
        }
    }
    print!("{:?}", arr);
}


// Passing args reference by reference

// fn double_negatives(a: &mut [i32; 10]) {
//     for i in 0..10 {
//         if (*a)[i] < 0 {
//             (*a)[i] *= 2;
//         }
//     }
// }

fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if (a)[i] < 0 {
            (a)[i] *= 2;
        }
    }
}