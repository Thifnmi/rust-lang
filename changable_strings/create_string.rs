fn main() {
    let s1 = String::new();
    let s2 = String::from("");
    let s3 = "".to_string();
    let s4 = "".to_owned();
    let s5 = format!("");
    print!("({}{}{}{}{})", s1, s2, s3, s4, s5);

    //
    let s = "a,";
    let s1 = String::from(s);
    let s2 = s.to_string();
    let s3 = s.to_owned();
    let s4 = format!(s);
    let s5 = format!("a,{}");
    let s6 = format!("{}", s);
    print!("({}{}{}{})", s1, s2, s3, s6);
}
