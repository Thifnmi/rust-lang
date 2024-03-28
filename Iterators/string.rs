fn main() {
    // let s = "abc012è€";
    // for i in 0..s.len() {
    //     println!("{}: {}", i, s.as_bytes()[i]);
    // }

    // // scan strings
    // fn print_nth_char(s: &str, mut n: u32) {
    //     let mut iter: std::str::Chars = s.chars();
    //     loop {
    //         let item: Option<char> = iter.next();
    //         match item {
    //             Some(c) => {
    //                 if n == 1 {
    //                     print!("{}", c);
    //                 }
    //             }
    //             None => {
    //                 break;
    //             }
    //         }
    //         n -= 1;
    //     }
    // }
    // print_nth_char("€èe", 3);

    // // print numeric code
    // fn print_codes(s: &str) {
    //     let mut iter = s.chars();
    //     loop {
    //         match iter.next() {
    //             Some(c) => {
    //                 println!("{}: {}", c, c as u32);
    //             }
    //             None => {
    //                 break;
    //             }
    //         }
    //     }
    // }
    // print_codes("€èe");

    // // Using Iterators in for Loops
    // fn print_codes_1(s: &str) {
    //     for c in s.chars() {
    //         println!("{}: {}", c, c as u32);
    //     }
    // }
    // print_codes_1("€èe");

    // // iterator with mutation
    // let slice1 = &[3, 4, 5];
    // let slice2 = &[7, 8];
    // let mut iterator = slice1.iter();
    // for item_ref in iterator {
    //     print!("[{}] ", *item_ref);
    // }
    // iterator = slice2.iter();
    // for item_ref in iterator {
    //     print!("({}) ", *item_ref);
    // }

    // // let mut slice = &mut [3, 4, 5];
    // // {
    // //     let mut iterator = slice.iter();
    // //     for mut item_ref in iterator {
    // //         *item_ref += 1;
    // //     }
    // // }
    // // print!("{:?}", slice);

    // let slice = &mut [3, 4, 5];
    // {
    //     let iterator = slice.iter_mut();
    //     for item_ref in iterator {
    //         *item_ref += 1;
    //     }
    // }
    // println!("{:?}", slice);

    // let slice = &mut [3, 4, 5];
    // {
    //     let iterator: std::slice::IterMut<i32> = slice.iter_mut();
    //     for item_ref in iterator {
    //         *item_ref += 1;
    //     }
    // }
    // print!("{:?}", slice);

    // for item_ref in (&mut [11u8, 22, 33]).iter_mut() {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in [44, 55, 66].iter_mut() {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // for item_ref in vec!['a', 'b', 'c'].iter_mut() {
    //     *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
    //     print!("{} ", *item_ref);
    // }

    // let slice: &mut [u8] = &mut [11u8, 22, 33];
    // let slice_it: std::slice::IterMut<u8> = slice.iter_mut();
    // for item_ref in slice_it {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // let mut arr: [i32; 3] = [44, 55, 66];
    // let arr_it: std::slice::IterMut<i32> = arr.iter_mut();
    // for item_ref in arr_it {
    //     *item_ref += 1;
    //     print!("{} ", *item_ref);
    // }
    // let mut vec: Vec<char> = vec!['a', 'b', 'c'];
    // let vec_it: std::slice::IterMut<char> = vec.iter_mut();
    // for item_ref in vec_it {
    //     *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
    //     print!("{} ", *item_ref);
    // }

    // An Iterator Adapter: filter

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter() {
        if *n < 0 {
            print!("{} ", n);
        }
    }

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().filter(|x| **x > 0) {
        print!("{} ", n);
    }

    // The map Iterator Adapter
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter() {
        print!("{} ", n * 2);
    }
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().map(|x| *x * 2) {
        print!("{} ", n);
    }

    // The enumerate Iterator Adapter
    let arr = ['a', 'b', 'c'];
    for i in 0..arr.len() {
        print!("{} {}, ", i, arr[i]);
    }

    let arr = ['a', 'b', 'c'];
    for ch in arr.iter() {
        print!("{}, ", ch);
    }

    let arr = ['a', 'b', 'c'];
    let mut i = 0;
    for ch in arr.iter() {
        print!("{} {}, ", i, *ch);
        i += 1;
    }

    let arr = ['a', 'b', 'c'];
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }

    // An Iterator Consumer: any
    println!("");
    let s = "Hello, world!";
    let ch = 'R';
    let mut contains = false;
    for c in s.chars() {
        if c == ch {
            contains = true;
        }
    }
    println!(
        "\"{}\" {} '{}'.",
        s,
        if contains {
            "contains"
        } else {
            "does not contain"
        },
        ch
    );

    let s = "Hello, world!";
    let ch = 'R';
    print!(
        "\"{}\" {} '{}'.",
        s,
        if s.chars().any(|c| c == ch) {
            "contains"
        } else {
            "does not contain"
        },
        ch
    );

    print!("{} ", [45, 8, 2, 6].iter().any(|n| *n < 0));
    print!("{} ", [45, 8, -2, 6].iter().any(|n| *n < 0));
    print!(
        "{} ",
        [45, 8, 2, 6].iter().any(|n: &i32| -> bool { *n < 0 })
    );
    print!(
        "{} ",
        [45, 8, -2, 6].iter().any(|n: &i32| -> bool { *n < 0 })
    );
}
