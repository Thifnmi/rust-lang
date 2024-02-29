fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    print!("{} {} {} {} {} {}", x[1], x[5], x[3], x[2], x[4], x[0]);

    let mut x = ["a", "b", "c"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    x = ["X", "Y", "Z"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    let y = ["1", "2", "3"];
    x = y;
    print!("{}{}{}.", x[0], x[1], x[2]);
    let _length = 6;
    let arr = [0; 6];
    print!("{}", arr[0]);

    let mut my_mutable_array = ["my", "array"];
    my_mutable_array[0] = "this is";
    print!("{} {}", my_mutable_array[0], my_mutable_array[1]);
}
