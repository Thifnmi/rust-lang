fn main() {
    let ss1 = "He";
    let ss2 = "llo ";
    let ds1 = ss1.to_string();
    let ds2 = ss2.to_string();
    let ds3 = format!("{}{}", ss1, ss2);
    println!("{}", ds3);
    let ds3 = format!("{}{}", ss1, ds2);
    println!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ss2);
    println!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ds2);
    println!("{}", ds3);

    //
    let mut dyn_str = "Hello".to_string();
    dyn_str = format!("{}{}", dyn_str, ", ");
    dyn_str = format!("{}{}", dyn_str, "world");
    dyn_str = format!("{}{}", dyn_str, "!");
    println!("{}", dyn_str);

    let mut dyn_str = "Hello".to_string();
    dyn_str.push_str(", ");
    dyn_str.push_str("world");
    dyn_str.push_str("!");
    println!("{}", dyn_str);

    //

    let mut dyn_str = "Hello".to_string();
    dyn_str += ", ";
    dyn_str += "world";
    dyn_str += "!";
    print!("{}", dyn_str);
}
