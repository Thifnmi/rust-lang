// use std::cmp::Ordering;
fn main() {
    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // println!("{:?}", arr.sort());
    // let desc = |a: &i32, b: &i32| -> Ordering {
    //     if a < b {
    //         Ordering::Greater
    //     } else if a > b {
    //         Ordering::Less
    //     } else {
    //         Ordering::Equal
    //     }
    // };
    // arr.sort_by(desc);
    // println!("{:?}", arr);

    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // arr.sort_by(|a, b| {
    //     if a < b {
    //         Ordering::Greater
    //     } else if a > b {
    //         Ordering::Less
    //     } else {
    //         Ordering::Equal
    //     }
    // });
    // println!("{:?}", arr);

    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // arr.sort();
    // println!("{:?}", arr);

    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // arr.sort_by(|a, b| a.cmp(b));
    // println!("{:?}", arr);

    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // arr.sort_by(|a, b| b.cmp(a));
    // println!("{:?}", arr);

    // other example

    let factor = 2;
    let multiply = |a| a * factor;
    println!("{}", multiply(13));
    let multiply_ref: &dyn Fn(i32) -> i32 = &multiply;
    println!(
        "{} {} {} {} {}",
        (*multiply_ref)(13),
        multiply_ref(13),
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13)
    );

    println!(
        "{}",
        (|v: &Vec<i32>| {
            let mut sum = 0;
            for i in 0..v.len() {
                sum += v[i];
            }
            sum
        })(&vec![11, 22, 34])
    );
}
