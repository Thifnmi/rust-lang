fn min(arr: &[i32]) -> i32 {
    // Let's assume 'arr' is not empty.
    let mut minimum = arr[0];
    for i in 1..arr.len() {
        if arr[i] < minimum {
            minimum = arr[i];
        }
    }
    minimum
}

fn main() {
    println!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));

    //
    let arr = [23, 17, 12, 16, 15, 2];
    let range = 2..5;
    let slice_ref = &arr[range];
    println!("{}", min(slice_ref));

    //
    let arr = [55, 22, 33, 44, 66, 7, 8];
    let v = vec![55, 22, 33, 44, 66, 7, 8];
    let sr1 = &arr[2..5];
    let sr2 = &v[2..5];
    println!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr1[1]);

    let arr = [55, 22, 33, 44, 66];
    let _r1 = 4..4;
    let _a1 = &arr[_r1];
    let _r2 = 4..3;
    //let _a2 = &arr[_r2];
    let _r3 = -3i32..2;
    //let _a3 = &arr[_r3];
    let _r4 = 3..8;
    //let _a4 = &arr[_r4];

    let mut arr = [11, 22, 33, 44];
    {
        let sl_ref = &mut arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref[1] = 0;
        print!(" {:?}", sl_ref);
    }
    print!(" {:?}", arr);

    //
    let arr = [11, 22, 33, 44];
    let n = 2;
    let sr1 = &arr[0..n];
    let sr2 = &arr[n..arr.len()];
    print!("{:?} {:?}", sr1, sr2);

    let sr1 = &arr[..n];
    let sr2 = &arr[n..];
    print!("{:?} {:?}", sr1, sr2);

    let r1: std::ops::RangeFrom<i32> = 3..;
    let r2: std::ops::RangeTo<i32> = ..12;
    println!(
        "{:?} {:?} {} {}",
        r1,
        r2,
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2)
    );
}
