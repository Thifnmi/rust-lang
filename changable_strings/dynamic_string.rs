fn main() {
    let mut a: String = "He".to_string();
    a.push('l');
    a.push('l');
    a.push('o');
    println!("{}", a);

    let mut b: String = "Xy".to_string();
    b.remove(0);
    b.insert(0, 'H');
    b.pop();
    b.push('i');
    println!("{}", b);

    let mut s1 = "".to_string();
    s1.push('e');
    let mut s2 = "".to_string();
    s2.push('è');
    let mut s3 = "".to_string();
    s3.push('€');
    println!("{} {}; ", s1.capacity(), s1.len());
    println!("{} {}; ", s2.capacity(), s2.len());
    println!("{} {}", s3.capacity(), s3.len());

    let mut s = "".to_string();
    for _ in 0..10 {
        println!("{:?} {} {}", s.as_ptr(), s.capacity(), s.len());
        s.push('a');
    }
    println!("{:?} {} {}: {}", s.as_ptr(), s.capacity(), s.len(), s);
}
