struct ExampleStruct {
    integer: i32,
    fractional: f32,
    character: char,
    five_bytes: [u8; 5],
}

fn main() {
    let data = ExampleStruct {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    print!("{}, {}, {}, {}\n", data.five_bytes[3], data.integer, data.fractional, data.character);

    let mut data_mutable = ExampleStruct {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    data_mutable.fractional = 1.99;
    print!("{}, {}, {}, {}\n", data_mutable.five_bytes[3], data_mutable.integer, data_mutable.fractional, data_mutable.character);
}
