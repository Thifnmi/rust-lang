fn main() {
    nomal();
    enu();
    match_construct();
}

fn nomal() {
    const ONE: u8 = 1;
    const TWO: u8 = 2;
    const THREE: u8 = 3;
    const FOUR: u8 = 4;
    const FIVE: u8 = 5;
    let number = ONE;
    if number == ONE {
        print!("1");
    } else if number == TWO {
        print!("2");
    } else if number == THREE {
        print!("3");
    } else if number == FOUR {
        print!("4");
    } else if number == FIVE {
        print!("5");
    }
}

fn enu() {
    enum Number {
        One,
        Two,
        Three,
        Four,
        Five,
    }
    let contin = Number::Three;
    match contin {
        Number::One => print!("\n1"),
        Number::Two => print!("\n2"),
        Number::Three => print!("\n3"),
        Number::Four => print!("\n4"),
        Number::Five => print!("\n5"),
    }
}

// fn convert_err() {
//     enum T {
//         A,
//         B,
//         C,
//         D,
//     }
//     let n: i32 = T::D;
//     let e: T = 1;
// }

fn match_construct() {
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }
    let mut contin = Continent::Europe;
    match contin {
        Continent::Europe => {
            contin = Continent::Asia;
            print!("\nE");
        }
        Continent::Asia => {
            let _a = 7;
        }
        Continent::Africa => print!("Af"),
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}

// fn invalid_expression() {
//     enum Number {
//         One,
//         Two,
//         Three,
//         Four,
//         Five,
//     }
//     let contin = Number::Three;
//     match contin {
//         Number::One => let a = 7;,
//         Number::Two => let a = 7,
//         Number::Three => print!("\n3"),
//         Number::Four => print!("\n4"),
//         Number::Five => print!("\n5"),
//     }
// }

fn compare() {
    enum CardinalPoint {
        North,
        South,
        West,
        East,
    }
    let direction = CardinalPoint::South;
    if direction == CardinalPoint::North {
    }
    if CardinalPoint::South < CardinalPoint::North {
    }
    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::South => print!("SOUTH"),
    }
}

fn enum_data() {
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(23.67);
    let outcome = Result::Failure(1200, 'X');
    match outcome {
        Result::Success(value) => print!("Result: {}", value),
        Result::Failure(error_code, module) =>
            print!("Error n. {} in module {}", error_code, module),
        Result::Uncertainty => {}
    }
}

fn match_is_expression() {
    enum CardinalPoint {
        North,
        South,
        West,
        East,
    }
    let direction = CardinalPoint::South;
    print!("{}", match direction {
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });
}
