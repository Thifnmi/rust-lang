fn main() {
    // _return_code();
    // _enviroment_var();
    // _command_line()
    _proper_runtime_error_handling()
}

fn _return_code() {
    std::process::exit(107);
}

fn _enviroment_var() {
    let command_line: std::env::Args = std::env::args();
    for argument in command_line {
        println!("[{}]", argument);
    }

    for var in std::env::vars() {
        println!("[{}]=[{}]", var.0, var.1);
    }

    println!("[{:?}]", std::env::var("abcd"));
    std::env::set_var("abcd", "This is the value");
    println!("[{:?}]", std::env::var("abcd"));

    println!("{}", if std::env::var("abcd").is_ok() { "Already defined" } else { "Undefined" });
    std::env::set_var("abcd", "This is the value");
    println!("{}.", match std::env::var("abcd") {
        Ok(value) => value,
        Err(err) => format!("Still undefined: {}", err),
    });
}

fn _command_line() {
    let mut line = String::new();
    println!("{:?}", std::io::stdin().read_line(&mut line)); println!("[{}]", line);

    // multi line
    let mut text = format!("First: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("Second: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {} bytes", text, text.len());

    // let mut text = format!("First: ");
    // let inp = std::io::stdin();
    // inp.read_line(&mut text);
    // text.push_str("Second: ");
    // inp.read_line(&mut text);
    // println!("{}: {} bytes", text, text.len());
}

fn _proper_runtime_error_handling() {
    fn f1(x: i32) -> Result<i32, String> {
        if x == 1 { Err(format!("Err. 1")) } else { Ok(x) }
    }
    fn f2(x: i32) -> Result<i32, String> {
        if x == 2 { Err(format!("Err. 2")) } else { Ok(x) }
    }
    fn f3(x: i32) -> Result<i32, String> {
        if x == 3 { Err(format!("Err. 3")) } else { Ok(x) }
    }
    fn f4(x: i32) -> Result<i32, String> {
        if x == 4 { Err(format!("Err. 4")) } else { Ok(x) }
    }
    fn f(x: i32) -> Result<i32, String> {
        match f1(x) {
            Ok(result) => {
                match f2(result) {
                    Ok(result) => {
                        match f3(result) {
                            Ok(result) => f4(result),
                            Err(err_msg) => Err(err_msg),
                        }
                    }
                    Err(err_msg) => Err(err_msg),
                }
            }

            Err(err_msg) => Err(err_msg),
        }
    }
    match f(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
}
