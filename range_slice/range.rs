fn main() {
    for i in 0..12 {
        print!("{}", i);
    }
    println!("");
    let dozen = 0..12;
    for j in dozen {
        print!("{}", j);
    }
    println!("");

    let range: std::ops::Range<usize> = 3..8;
    println!(
        "{:?}, {}, {}, {}",
        range,
        range.start,
        range.end,
        range.len()
    );
    for i in range {
        print!("{}, ", i);
    }
    println!("");

    let r1 = 3u8..12u8;
    let r2 = 3u8..12;
    let r3 = 3..12u8;
    let r4 = 3..12;
    let r5 = -3..12;
    let r6 = 3..12 as i64;
    println!(
        "{} {} {} {} {} {}",
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2),
        std::mem::size_of_val(&r3),
        std::mem::size_of_val(&r4),
        std::mem::size_of_val(&r5),
        std::mem::size_of_val(&r6)
    );
    println!("{}", min([23, 17, 12, 16, 15, 28, 17, 30]));
    println!("{}", min_drawback1(&[23, 17, 12, 16, 15, 28, 17, 30]));
    println!("{}", min_drawback4(&[23, 17, 12, 16, 15, 28, 17, 30]));
}

fn min(arr: [i32; 8]) -> i32 {
    let mut minimum = arr[0];
    for i in 1..arr.len() {
        if arr[i] < minimum {
            minimum = arr[i];
        }
    }
    minimum
}

fn min_drawback1(arr: &[i32; 8]) -> i32 {
    let mut minimum = arr[0];
    for i in 1..arr.len() {
        if arr[i] < minimum {
            minimum = arr[i];
        }
    }
    minimum
}

fn min_drawback4(arr: &[i32; 8], start: usize, count: usize) -> i32 {
    // Let's assume 'start' is between 0 and 7,
    // and 'count' is between 1 and 8 - start.
    let mut minimum = arr[start];
    for i in start + 1..start + count {
        if arr[i] < minimum {
            minimum = arr[i];
        }
    }
    minimum
}
