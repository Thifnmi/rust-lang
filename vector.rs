fn main() {
    let x = vec!["This", "is"];
    println!("{} {}. Length: {}.", x[0], x[1], x.len());

    let mut x = vec!["This", "is"];
    print!("{}", x.len());
    x.push("a");
    print!(" {}", x.len());
    x.push("sentence");
    print!(" {}", x.len());
    x[0] = "That";
    for i in 0..x.len() {
        print!(" {}", x[i]);
    }

    let length = 5000;
    // vec![<default value>; <length>]
    let mut y = vec![4; length];
    y[6] = 3;
    y.push(6);
    print!("\n{}, {}, {}, {}", y[0], y[6], y[4999], y[5000]);

    let mut _x = vec!["a", "b", "c"];
    _x = vec!["X", "Y"];

    let mut _x = vec!["a", "b", "c"];
    _x = vec![15, 16, 17];
}
