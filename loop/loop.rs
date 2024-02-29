fn main() {
    let index = 8;
    for index in 0..4 {
        print!("{} ", index);
    }
    print!(":{}", index);

    let mut limit = 4;
    for i in 1..limit {
        limit -= 1;
        print!("{limit}");
        print!("{} ", i);
    }
    print!(":{}", limit);
}
