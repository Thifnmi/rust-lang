fn main() {
    let s = "abc012è€";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }

    // scan strings
    fn print_nth_char(s: &str, mut n: u32) {
        let mut iter: std::str::Chars = s.chars();
        loop {
            let item: Option<char> = iter.next();
            match item {
                Some(c) => {
                    if n == 1 {
                        print!("{}", c);
                    }
                }
                None => {
                    break;
                }
            }
            n -= 1;
        }
    }
    print_nth_char("€èe", 3);


    // print numeric code
    fn print_codes(s: &str) {
        let mut iter = s.chars();
        loop {
            match iter.next() {
                Some(c) => {
                    println!("{}: {}", c, c as u32);
                }
                None => {
                    break;
                }
            }
        }
    }
    print_codes("€èe");

    // Using Iterators in for Loops
    fn print_codes_1(s: &str) {
        for c in s.chars() {
            println!("{}: {}", c, c as u32);
        }
    }
    print_codes_1("€èe");
}
