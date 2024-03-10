use std::mem::*;
// use std::mem::size_of;
// use std::mem::size_of_val;

fn main() {
    // println!("{} ", std::mem::size_of::<i32>());
    // println!("{} ", std::mem::size_of_val(&12));

    // size của các kiểu dữ liệu nguyên thủy
    // println!(
    //     "{} {} {} {} {} {} {} {} {} {} {} {}",
    //     size_of::<i8>(),
    //     size_of::<u8>(),
    //     size_of::<i16>(),
    //     size_of::<u16>(),
    //     size_of::<i32>(),
    //     size_of::<u32>(),
    //     size_of::<i64>(),
    //     size_of::<u64>(),
    //     size_of::<f32>(),
    //     size_of::<f64>(),
    //     size_of::<bool>(),
    //     size_of::<char>()
    // );

    // vài size của kiểu dữ liệu khác sẽ phụ thuộc vào nền tảng thiết bị đang biên dịch
    // println!(
    //     "{} {} {} {}",
    //     size_of::<isize>(),
    //     size_of::<usize>(),
    //     size_of::<&i8>(),
    //     size_of::<&u32>()
    // );

    // Representation của kiểu dữ liệu nguyên thủy
    println!("{:?}", as_bytes(&1i8));
    println!("{:?}", as_bytes(&2i16));
    println!("{:?}", as_bytes(&3i32));
    println!("{:?}", as_bytes(&(4i64 + 5 * 256 + 6 * 256 * 256)));
    println!("{:?}", as_bytes(&'A'));
    println!("{:?}", as_bytes(&true));
    println!("{:?}", as_bytes(&&1i8));

    // location of bytes in memory

    let b1 = true;
    let b2 = true;
    let b3 = false;
    println!(
        "{} {} {}",
        &b1 as *const bool as usize, &b2 as *const bool as usize, &b3 as *const bool as usize
    );

    // size of composite data type
    enum E1 {
        E1a,
        E1b,
    };
    enum E2 {
        E2a,
        E2b(f64),
    };
    println!(
        "{} {} {} {} {} {}",
        size_of_val(&[0i16; 80]),
        size_of_val(&(0i16, 0i64)),
        size_of_val(&[(0i16, 0i64); 100]),
        size_of_val(&E1::E1a),
        size_of_val(&E2::E2a),
        size_of_val(&vec![(0i16, 0i64); 100])
    );

    vector_allocation2();
}

fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(o as *const _ as *const u8, std::mem::size_of::<T>()) }
}

fn vector_allocation() {
    let mut v = vec![0; 0];
    println!("{} {}", v.len(), v.capacity());
    v.push(11);
    println!("{} {}", v.len(), v.capacity());
    v.push(22);
    println!("{} {}", v.len(), v.capacity());
    v.push(33);
    println!("{} {}", v.len(), v.capacity());
    v.push(44);
    println!("{} {}", v.len(), v.capacity());
    v.push(55);
    println!("{} {}", v.len(), v.capacity());
}

fn vector_allocation2() {
    let mut v = vec![0; 0];
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1_000 {
        let cap = v.capacity();
        if cap != prev_capacity {
            println!("{} {} {}", i, v.len(), cap); prev_capacity = cap;
        }
        v.push(1); }
}
