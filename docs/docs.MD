check rust version `rustc -V`

rust code file name `<file_name>.rc`

`rustc <file_name>.rc`  để build

windows <file_name>.exe,  2 lần vào file để chạy hoặc gõ tên file trên terminal

OS khác <file_name>, ./<file_name> trên terminal để chạy

The smallest valid Rust program is:
fn main(){}

fn  function
main tên function

fn main(){} function rỗng tên main

command line script debug

```
rustc $* --color always 2>&1 | more
```

# **Data types**

## *string*

```
let x = "my string"
```

## *integer*

```
let number = 12345
```

## *float*

```
let f = 12.3
let f = 12.
```

## *bool*

```
let b = true
let b = false
```

## *array*

```
let my_array = ["my", "array"]
```

## *mutable array* (array in array)

```
let mut my_mutable_array = ["my", "array"];
my_mutable_array[0] = ["this is"];
print!("{} {}.", my_mutable_array[0], my_mutable_array[1]);

/*
result:
this is array.
*/
```

## *Multidimensional Arrays* (Mảng đa chiều)

```
let multidimensional_arrays = [[[[]]]]
```

## *Vector*

```
let x = vec!["This", "is"];
```

*rust cannot mix interger number with float number*

```
println!("{}", 2.5 + 1)
```

*fixed with add a dot (.)*

```
println!("{}", 2.5 + 1.)
```

# **operators**

```
< is less than
> is greater than
== is equal to
!= is different from
<= is less than or equal to
>= is greater than or equal to
```

# **conditional statements (if)**

```
if condition {
    //exec code
}
```

```
if condition {
    // if logic
}
else {
    // else logic
}
```

```
if condition {
    // if logic
}
else if condition{
    // else if logic
}
else {
    // else logic
}
```

# **expression**

```
print!("{}",
    if condition {
        "value 1"
    }
    else if condition {
        "value 2"
    }
    else if condition {
        "value 3"
    }
    else {
        "value 4"
    }
);
```

# **loop**

## *conditioned loop*

```
while condition {
    // logic
}
```

## *infinite loop*

```
while true {
    // logic
}
```

## *couting loop (for)*

```
for variable in a..b  {
    // logic
}
```

there is the in keyword, followed by two integer numeric expressions, separated by the symbol ".."

# **array**

```
let `<mut>` _arr = ["value 1", "value 2"];
```

## *Arrays of Specified Size*

```
let `<mut> `_arr = [`<default value>`; `<length>`];
```

## *Multidimensional Arrays*

```
let `mut` arr = [[[[["ahihi"], 100], 17], 5], 20];
```

default value is "ahihi", first statement declare an arr of 20 items

check len array

```
print!("{}, {}, {}, {}.", arr.len(), arr[0].len(), arr[0][0].len(), arr[0][0][0].len(), arr[0][0][0][0].len());
```

```
let length = 6;

let arr = [0; length];
```

# **vector**

```
let vt = vec![default_value; length];
```

Vectors allow us to do everything that is allowed for arrays, but they allow also us to change their size after having been initialized

add item to vector:

```
vt.push("value"); // same stack, it push data to latest position

vt[index] = "value"; // same array
```

Định nghĩa vector với chuỗi giá trị bằng 4. (float) và có độ dài là 500. kiểu khai báo này không thể thực hiện với array

```
let length = 500;
let mut y = vec![4.; length];
y[6] = 3.14;
y.push(4.89);
print!("{}, {}, {}", y[6], y[499], y[500]);

/*
Kết quả:
3.14, 4, 4.89
*/
```

không giống với array, độ dài của vector có thể thay đổi được trong quá trình runtime tuy nhiêu kiểu giá trị không thể thay đổi. trong rust giá trị của biến chỉ khai báo 1 lần khi biên dịch

```
let mut _x = vec!["a", "b", "c"];
_x = vec!["X", "Y"];
let mut _x = vec!["a", "b", "c"];
_x = vec![15, 16, 17];
```

các phương thức khác với vector

```
let mut x = vec!["This", "is", "a", "sentence"];
x.insert(1, "line");
x.insert(2, "contains");
x.remove(3);
x.push("about Rust");
x.pop();
for i in 0..x.len()
{
   print!("{} ", x[i]);
}


/*
Kết quả:
This line contains a sentence.
*/
```

# **Using Primitive Types**

## *Non-Decimal Numeric Bases*

```
let hexadecimal = 0x10;
let decimal = 10;
let octal = 0o10;
let binary = 0b10;
print!("{} {} {} {}", hexadecimal, decimal, octal, binary);

/*
Kết quả:
16 10 8 2
*/
```

## *Underscore in Numeric Literals (_)*

```
let one_billion = 1_000_000_000;

let x = 3___4_.56_;

print!("{}, {}", one_billion, x);

/*
kết quả:
1000000000, 34.56
*/


let hexadecimal = 0x_00FF_F7A3;
let decimal = 1_234_567;
let octal = 0o_777_205_162;
let binary = 0b_0110_1001_1111_0001; print!("{} {} {} {}", hexadecimal, decimal, octal, binary);


/*
Kết quả:
16775075 1234567 134023794 27121
*/
```

# **Number**

## *Signed Integer Numbers* // Số nguyên có dấu

có 2 loại là integer và float

integer trong Rust chia thành 8-bit signed integer, 16-bit signed integer, 32-bit signed integer, 64-bit signed integer

```
let a: i8 = 5;
let b: i16 = 5;
let c: i32 = 5;
let d: i64 = 5;
```

i8 cho phép biểu diễn 256 giá trị tương đương 2^8 và trong khoảng -127 -> +127

chỉ có thể thực hiện biểu thức toán tử giữa các biến cùng kiểu giá trị

## *Unsigned Integer Numbers* // Số nguyên không dấu

Giống như signed integer, unsigned integer cũng có các dạng 8-bit unsigned integer, 16-bit unsigned integer, 32-bit unsigned integer, 64-bit unsigned integer
u8 cũng cho phép biểu diễn 256 giá trị giống như i8 tuy nhiên chỉ cho phép giá trị nguyên dương và trong khoảng từ 0 -> 255

Mục đích của các loại số nguyên phụ thuộc vào mục đích sử dụng

• on 16-bit computers, it is an unsigned 16-bit integer;
• on 32-bit computers, it is an unsigned 32-bit integer;
• on 64-bit computers, it is an unsigned 64-bit integer.

index của array hay vector phải là kiểu dữ liệu unsigned integer vì nó giống với địa chỉ giá trị trong RAM và cũng chỉ có kiểu dữ liệu này được phép sử dụng

Rust không hỗ trợ 16-bit system nhưng lại hỗ trợ cả 32-bit system và 64-bit system

Việc lựa chọn 32-bit hay 64-bit không liên quan tới việc tối ưu code mà nó phụ thuộc vào việc sau khi biên dịch sẽ chạy trên hệ thống nào? (32-bit hay 64-bit). Với mục đích đó, Rust có thêm 2 loại là isize và usize

## *Float*
Trong rust có 2 kiểu float là f32 và f64 tương đương với 32-bit floating-point numbers và 64-bit floating-point numbers
f32 hiển thị 7 số phần thập phân, f64 hiển thị 16 số phần thập phân

# **Explicit Conversions**


```
let a: i16 = 12;
let b: u32 = 4;
let c: f32 = 3.7;
print!("{}", a as i8 + b as i8 + c as i8);


/*
Kết quả:
19
*/
```
sử dụng toán tử `as` để có thể convert

ở ví dụ trên khi convert biến `c` có giá trị 3.7 kiểu f32 qua kiểu i8 thì phần nguyên của giá trị là 3 nên kết quả đạt được sau convert là 3 kiểu dữ liệu i8

với 1 ví dụ khác
```
let a = 500 as i8;
let b = 100_000 as u16;
let c = 10_000_000_000 as u32;
print!("{} {} {}", a, b, c);

/*
Kết quả:
-12 34464 1410065408
*/
```

lúc này giá trị của biến a = -12 vì 500 không thể dùng 8 bit để biểu diễn (2^8 = 256) mà phải dùng ít nhất 9 bit (2^9=512) để biểu diễn ...
tương tự ...

## *Type suffixes of numeric literals*

```
let _a: i16 = -150;
let _b = -150 as i16;
let _c = -150 + _b - _b;
let _d = -150i16;
let _e = -150_i16;
print!("\nsuffixes of number _a {}, _b {}, _c {}, _d {}, _e {}.", _a, _b, _c, _d, _e);

/*
Kết quả:
suffixes of number _a -150, _b -150, _c -150, _d -150, _e -150.
*/
```

Kiểu 1: khai báo biến có kiểu dữ liệu và gán giá trị
kiểu 2: khai báo biến và giá trị rồi convert qua kiểu dữ liệu
kiểu 3: khai báo biến và giá trị rồi kiểu giá trị ngay sau (không có khoảng trắng space) hoặc thêm dấu underscore "_"

Tương tự với kiểu dữ liệu float

# **All the Numeric Types**

```
let _: i8 = 127;
let _: i16 = 32_767;
let _: i32 = 2_147_483_647;
let _: i64 = 9_223_372_036_854_775_807;
let _: isize = 100; // The maximum value depends on the target architecture
let _: u8 = 255;
let _: u16 = 65_535;
let _: u32 = 4_294_967_295;
let _: u64 = 18_446_744_073_709_551_615;
let _: usize = 100; // The maximum value depends on the target architecture
let _: f32 = 1e38;
let _: f64 = 1e308;
```

```
Type    | Occupied bytes | Minimum value                           |  Maximum value
i8      |     1          |   -128                                  |     +127
i16     |     2          |   -32,768                               |     +32,767
i32     |     4          |   -2,147,483,648                        |     +2,147,483,647
i64     |     8          |   -2^63                                 |     +(2^63)-1
isize   |     4 or 8     |   on a 32-bit target: -2,147,483,648;   |     on a 32-bit target: +2,147,483,647
        |                |   on a 64-bit target: -2^63             |     on a 64-bit target: +(2^63)-1
u8      |     1          |   0                                     |     +255
u16     |     2          |   0                                     |     +65,535
u32     |     4          |   0                                     |     +4,294,967,295
u64     |     8          |   0                                     |     +(2^64)-1
usize   |     4 or 8     |   0                                     |     on a 32-bit target: +4,294,967,295
        |                |                                         |     on a 64-bit target: +(2^64)-1

just two floating-point numeric types:
• f32, having 32 bits, is equivalent to the float type of the C language.
• f64, having 64 bits, is equivalent to the double type of the C language.

```


# **Booleans and Characters**

```
let a: bool = true;
print!("[{}]", a);
let b: char = 'a';
print!("[{}]", b);

/*
Kết quả:
[true][a]
*/

```

```
let e_grave = 'è';
let japanese_character = 'さ';
println!("{} {}", e_grave, japanese_character);

/*
Kết quả:
è さ
*/
```

```
let _a = 'a' + 'b';
let _b = false + true;

/*
Kết quả:

error[E0369]: cannot add `char` to `char`
  |     let _a = 'a' + 'b';
  |              --- ^ --- char
  |              |
  |              char

error[E0369]: cannot add `bool` to `bool`
  |
  |     let _b = false + true;
  |              ----- ^ ---- bool
  |              |
  |              bool
*/
```

Điều này có nghĩa là trong Rust cả bool và char để không được coi là số nên việc thực hiện các toán tử với chúng là vi phạm qui tắc

thay vào đó có thể convert chúng qua number

```
print!("{} {} {} {} {}", true as u8, false as u8, 'A' as u32, 'à' as u32, '€' as u32);

/*
Kết quả:
1 0 65 224 8364
*/
```

trong đó true/false là 1 và 0, các ký tự được convert về Hex trong bảng mã ascii
ngược lại, ta có thể convert các dữ liệu dạng số nguyên qua ký tự

```
print!("{} {}",65 as char, 224 as char);

/*
Kết quả:
A à
*/
```

# **The Empty Tuple**

....


# **Array and Vector Types**

```
let _array1: [char; 3] = ['x', 'y', 'z'];
let _array2: [f32; 200] = [0f32; 200];
let _vector1: Vec<char> = vec!['x', 'y', 'z'];
let _vector2: Vec<i32> = vec![0; 5000];
```

# **Constants**

```
let n = 20;
let _ = [0; n];

/*
Kết quả:
error: expected item, found keyword `let`
  |
  | let n = 20;
  | ^^^ consider using `const` or `static` instead of `let` for global variables

error: aborting due to previous error
*/
```

Cách làm trên đã vi phạm qui tắc trong Rust. Bởi độ dài của array phải được làm rõ trước khi biên dịch ngay cả khi nó là immutable (giá trị bất biến, không thể thay đổi khi gán vào biến), do đó theo 1 nghĩa nào đó có thể hiểu giá trị khi biên dịch là 1 hằng số (constants)

```
const N: usize = 20;
let _ = [0; N];
```


# **Discovering the Type of an Expression**

....


# **Enumerating Cases**

## *Enumerations*

Instead of writing the following code:

```
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

/*
Kết quả:
1
*/
```

It is better to write the following equivalent code:

```
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

/*
Kết quả:
3
*/
```

enums chỉ có thể có 1 giá trị trong list được định nghĩa trước. Những items đó được gọi là variants (biến thể)

```
enum T {A, B, C, D};
let n: i32 = T::D;
let e: T = 1;


/*

error[E0308]: mismatched types
   |
   |     let n: i32 = T::D;
   |            ---   ^^^^ expected `i32`, found `T`
   |            |
   |            expected due to this

error[E0308]: mismatched types
   |
   |     let e: T = 1;
   |            -   ^ expected `T`, found integer
   |            |
   |            expected due to this
*/
```

enums không thể ngầm convert qua numbers và ngược lại numbers không thể ngầm convert qua enums

## *The match Construct*

The `match` statement is the basic Rust tool to use enumerations, similarly to the `switch` statement in the C language, even if they differ about many aspects.

Các biểu thức (expressions) ở sau keyword `match` không nhất thiết phải nằm trong dấu ngoặc kép (enclosed). Các trường hợp khác nhau được tạo từ pattern sau đó là ký tự `=>` và cuối cùng là biểu thức, những trường hợp này được ngăn cách bởi dấu phẩy (commas)
ví dụ:
```
enum Var {
    Val1,
    Val2,
}
let x = Var::Val2;
match x {
    Var::Val1 => print!("hello"),
    Var::Val2 => print!("hi"),
}
```
`print!()` là 1 biểu thức hợp lệ. Tuy nhiên `let a = 3;`, `let a = 3` hay `fn empty() {}` không phải biểu thức hợp lệ


```
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
}
let contin = Number::Three;
match contin {
    Number::One => let a = 7;,
    Number::Two => let a = 7,
    Number::Three => fn name() {},
    Number::Four => print!("\n4"),
    Number::Five => print!("\n5"),
}
```

Nếu muốn sử dụng biểu câu lệnh không phải biêủ thức hay đánh giá một số biểu thức ở sau `=>` có thể sử dụng block `{}`

```
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
        print!("E");
    }
    Continent::Asia => {
        let a = 7;
    }
    Continent::Africa => print!("Af"),
    Continent::America => print!("Am"),
    Continent::Oceania => print!("O"),
}

/*
Kết quả:
E
*/
```

## *Relational Operators and Enums*

enums không thể so sánh với toán tử `==` 

```
enum CardinalPoint {
    North,
    South,
    West,
    East,
}
let direction = CardinalPoint::South;
if direction == CardinalPoint::North {
}

/*


error[E0369]: binary operation `==` cannot be applied to type `CardinalPoint`
    |
    |     if direction == CardinalPoint::North {
    |        --------- ^^ -------------------- CardinalPoint
    |        |
    |        CardinalPoint
*/
```
enums được dùng trong rất nhiều thư viện và cả trong các thư viện của Rust vì vậy không chỉ toán tử `==` mà các toán tử quan hệ khác cũng bị coi là không hợp lệ


```
enum CardinalPoint {
    North,
    South,
    West,
    East,
}
let direction = CardinalPoint::South;
if CardinalPoint::South < CardinalPoint::North { }

/*
error[E0369]: binary operation `<` cannot be applied to type `CardinalPoint`
    |
    |     if CardinalPoint::South < CardinalPoint::North { }
    |        -------------------- ^ -------------------- CardinalPoint
    |        |
    |        CardinalPoint
*/

```

## *Handling All the Cases*

```
enum CardinalPoint {
    North,
    South,
    West,
    East,
}
let direction = CardinalPoint::South;
match direction {
    CardinalPoint::North => print!("NORTH"),
    CardinalPoint::South => print!("SOUTH"),
}

/*
error[E0004]: non-exhaustive patterns: `CardinalPoint::West` and `CardinalPoint::East` not covered
    |
    |     match direction {
    |           ^^^^^^^^^ patterns `CardinalPoint::West` and `CardinalPoint::East` not covered
    |
note: `CardinalPoint` defined here
    |
    |     enum CardinalPoint {
    |          -------------
...
    |         West,
    |         ^^^^ not covered
    |         East,
    |         ^^^^ not covered
    = note: the matched value is of type `CardinalPoint`
*/
```

Để có thể biên dịch được, cần phải handle hết các case

```
enum CardinalPoint {
    North,
    South,
    West,
    East,
}
let direction = CardinalPoint::South;
match direction {
    CardinalPoint::North => print!("NORTH"),
    CardinalPoint::South => print!("SOUTH"),
    CardinalPoint::East => {},
    CardinalPoint::West => {},
}
```

Đây là 2 biến nhưng nếu có nhiều biến hơn thì sao? để tránh phải handle hết các biến không có tác động có thể dùng `_` (underscore)

```
enum CardinalPoint {
    North,
    South,
    West,
    East,
}
let direction = CardinalPoint::South;
match direction {
    CardinalPoint::North => print!("NORTH"),
    CardinalPoint::South => print!("SOUTH"),
    _ => {},
}
```

`_` luôn match với tất cả giá trị vì vậy để tránh bị lỗi trong quá trình biên dịch cũng như thực thi cần handle hết các case cần thiết trước. `_` giống như `default` trong C

## *Using match with Numbers*


```

match "value" {
    "val" => print!("value "),
    _ => print!("other "),
}
match 3 {
    3 => print!("three "),
    4 => print!("four "),
    5 => print!("five "),
    _ => print!("other "),
}
match '.' {
    ':' => print!("colon "),
    '.' => print!("point "),
    _ => print!("other "),
}

/*
other three point
*/

```

# *Enumerations with Data*

```
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

/*

Error n. 1200 in module X
*/
```

## *“match” Expressions*

giống như `if`, `match` cũng là biểu thức 

```
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

/*

S
*/
```

Nếu thay biểu thức thứ 3 thành `_ => {},` có thể sẽ gặp lỗi `match arms have incompatible types`

## *Use of Guards in match Constructs*

```
for n in -2..5 {
    println!("{} is {}.", n, match n {
        0 => "zero",
        1 => "one",
        _ if n < 0 => "negative",
        _ => "plural",
    });
}

/*

-2 is negative.
-1 is negative.
0 is zero.
1 is one.
2 is plural.
3 is plural.
4 is plural.
*/
```

# **Using Heterogeneous Data Structures** Cấu trúc dữ liệu không đồng nhất

## *Tuples*

Không giống như arrays hay vector, tuple có thể lưu trữ đối một hay nhiều object không có cùng kiểu dữ liệu

```
let data = (10000000, 183.19, 'Q');
let copy_of_data = data;
print!("{}, {}, {}", data.0, copy_of_data.1, data.2);


/*
10000000, 183.19, Q
*/
```

ngoài ra data type của tuples có thể được định nghĩa rõ ràng 

```
let data: (i32, f64, char) = (10000000, 183.19, 'Q');
```

giống như array, tuple cũng có thể truy vấn dữ liệu từng item bên trong bằng cách

```
data.<index>
```

Và dữ liệu của tuple cũng có thể thay đổi được

```
let mut data = (10000000, 183.19, 'Q'); data.0 = -5;
data.2 = 'x';
print!("{}, {}, {}", data.0, data.1, data.2);


/*
-5, 183.19, x
*/
```

Điểm khác nhau giữa array và tuple là tuple không thể truy cập bằng variable index


```
let array = [12, 13, 14];
let tuple = (12, 13, 14);
let i = 0;
print!("{}", array[i]);
print!("{}", tuple.i);

/*

error[E0609]: no field `i` on type `({integer}, {integer}, {integer})`
  |
  |     print!("{}", tuple.i);
  |                        ^
*/
```

## *Structs*

tuples rất hữu ích khi không có quá nhiều item, nhưng khi có nhiều thì rất dễ nhầm lẫn và code trở nên khó hiểu hơn

```
let data = (10, 'x', 12, 183.19, 'Q', false, -9);
print!("{}", data.2 + data.6);
                ^        ^
                12      -9
/*

3
*/
```

Ngoài ra kiểu dữ liệu của từng ittem trong tuple được định nghĩa bởi chuỗi các field, nếu nhiều field thì sẽ quá dài để định nghĩa và khó để hiểu

```
let data1 = (10, 'x', 12, 183.19, 'Q', false, -9);
let mut data2: (u16, char, i16, f64, bool, char, i16);
data2 = data1;

/*
Đoạn code trên lỗi do nhầm kiểu giá trị giữa index 4 và 5
*/
```
Ngoài ra nếu thêm 1 field vào đầu của tuple thì tất cả index object của tuple phải tăng thêm 1 trong code ví dụ `data.2` phải tăng lên thành `data.3`

Do đó, sẽ rất hữu ích nếu có câu lệnh định nghĩa cấu trúc, khai báo tên, gán nhãn cho các field của cấu trúc đó

```
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
    print!("{}, {}, {}, {}", data.five_bytes[3], data.integer, data.fractional, data.character);
}

/*

60, 10000000, 183.19, Q
*/
```

Nếu variable được định nghĩa là mutable thì giá trị các trường của biến có thể được thay đổi 

```
struct ExampleStruct {
    integer: i32,
    fractional: f32,
}
let mut data = ExampleStruct {
    integer: 10,
    fractional: 183.19,
};
data.fractional = 8.2;
print!("{}, {}", data.fractional, data.integer);

/*
8.2, 10
*/
```

Giống như tuple, vì vậy có thể khỏi tạo empty structs

## *Tuple-Structs*

Là sự kết hợp giữa `tuples` và `structs`. kiểu cấu trúc này có tên, phải được định nghĩa trước giống như `structs` nhưng lại không có tên các `fields` giống `tuples`

```
struct SomeData (
    i32,
    f32,
    char,
    [u8; 5],
);
let data = SomeData (
    10_000_000,
    183.19,
    'Q',
    [9, 0, 250, 60, 200],
);
print!("{}, {}, {}, {}", data.2, data.0, data.1, data.3[2]);
/*

Q, 10000000, 183.19, 250
*/
```

## *Lexical Conventions*

```
const MAXIMUM_POWER: u16 = 600;
enum VehicleKind {
    Motorcycle,
    Car,
    Truck,
}
struct VehicleData {
    kind: VehicleKind,
    registration_year: u16,
    registration_month: u8,
    power: u16,
}
let vehicle = VehicleData {
    kind: VehicleKind::Car,
    registration_year: 2003,
    registration_month: 11,
    power: 120,
};
if vehicle.power > MAXIMUM_POWER {
    println!("Too powerful");
}
```

    - Tên của biến constaints chỉ cho phép khí tự viết hoa và được nối với nhau bằng dấu underscore `_`
    - Tên hàm, thư viện thông thường và enum variant bao gôm các từ được ghép với nhau và viết hoa chứ cái đầu tiên mỗi từ (Camel style convention)
    - Các biến khác chỉ sử dụng chữ cái viết thường, giữa các từ nối với nhau bằng dấu underscore `_`


# **Defining Functions**

## *Defining and Invoking a Function*


```
fn line() {
    println!("----------");
}
line();
line();
line();

/*
----------
----------
----------
*/
```

## *Functions Defined After Their Use*

Not use with variable
```
f();
fn f() {}
```

## *Functions Shadowing Other Functions*

```
{
    fn f() {
        print!("a");
    }
    f();
    f();
}
{
    f();
}

/*

aab
*/
```

```
fn f() {
    print!("1");
}
fn main() {
    f();
    f();
    fn f() {
        print!("3");
    }
}

f();
fn f() {
    print!("2");
}
/*
232
*/
```

## *Passing Arguments to a Function*

```
fn print_sum(addend1: f64, addend2: f64) {
    println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
}
fn main() {
    print_sum(3.0, 5.0);
    print_sum(3.2, 5.1);
}
/*

3 + 5 = 8
3.2 + 5.1 = 8.3
*/
```


## *Passing Arguments by Value*


```
fn print_double(mut x: f64) {
    x *= 2.0;
    println!("value in func print_double {}", x);
}
fn main() {
    let x = 4.0;
    print_double(x);
    print!("{}", x);
}
```

## *Returning a Value from a Function*

```
fn double(x: f64) -> f64 {
    x * 2.0
}
fn main() {
    print!("{}", double(17.3));
}

/*
34.6
*/
```

```
fn f1(x: i32) {}
fn f2(x: i32) -> () {}
```

`f1` `f2` is a same function, both return an empty tuple

This code is valid
```
fn f1() -> i32 { 4.5; "abc"; 73i32 }
fn f2() -> i32 { 4.5; "abc"; 73 }
fn f3() -> i32 { 4.5; "abc"; 73 + 100 }
```
but this code is not

```
fn f1() -> i32 { 4.5; "abc"; false }
fn f2() -> i32 { 4.5; "abc"; () }
fn f3() -> i32 { 4.5; "abc"; {} }
fn f4() -> i32 { 4.5; "abc"; }

/*

`mismatched type errors`
f1: expected i32, found bool
f2,f3,f4: expected i32, found ()
*/
```

## *Early Exit*

```
fn f(x: f64) -> f64 {
    if x <= 0. {
        return 0.;
    }
    x + 3.
}
print!("{} {}", f(1.), f(-1.));

/*

4 0
*/
```

```
fn f(x: i32) {
    if x <= 0 { return; }
    if x == 4 { return (); }
    if x == 7 { return {}; }
    print!("{}", x);
}
f(5);
```

## *Returning Several Values* // 1 func trả về nhiều giá trị

có thể dùng tuple

```
fn several_values(dividend: i32, divisor: i32) -> (i32, i32){
    (dividend / divisor, dividend % divisor)
}
println!("{:?}", several_values(12, 9))

/*

(1, 3)
*/
```

hoặc có thể trả về enum, struct, tuple struct, array, vector

```
enum E { E1, E2 }
struct S { a: i32, b: bool }
struct TS (f64, char);
fn f1() -> E { E::E2 }
fn f2() -> S { S { a: 49, b: true } }
fn f3() -> TS { TS (4.7, 'w') }
fn f4() -> [i16; 4] { [7, -2, 0, 19] }
fn f5() -> Vec<i64> { vec![12000] }
print!("{} ", match f1() { E::E1 => 1, _ => -1 }); print!("{} ", f2().a);
print!("{} ", f3().0);
print!("{} ", f4()[0]);
print!("{} ", f5()[0]);
/*
-1 49 4.7 7 12000
*/
```

## *Change a Variable of the Caller*

Thông thường 

```
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
for i in 0..10 {
    if arr[i] < 0 {
        arr[i] *= 2;
    }
}
print!("{:?}", arr);

/*
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]
*/
```

Chuyển sang thành func để tái sử dụng

```
fn double_negatives(mut a: [i32; 10]) { for i in 0..10 {
    if a[i] < 0 {
        a[i] *= 2;
        }
    }
}
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
double_negatives(arr);
print!("{:?}", arr);

/*
[5, -4, 9, 0, -7, -1, 3, 5, 3, 1]
*/
```

Kết qủa là array vẫn được giữ nguyên. bởi vì khi gọi hàm giá trị của array đã được sao chép qua array khác. khi hàm xử lý xong các giá trị được sao chép (cũng đã qua xử lý) cũng sẽ bị hủy vì vậy giá trị của hàm gốc vẫn còn nguyên và cũng có thể bỏ khai báo `mut` khi khai báo mảng. Cách khắc phục

```
fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        if a[i] < 0 {
            a[i] *= 2;
        }
    }
    a
}
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
arr = double_negatives(arr);
print!("{:?}", arr);

/*
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]
*/
```

Kết quả lúc này đã chính xác. tuy nhiên có 1 nhược điểm là tất cả data được copy qua. func được gọi sẽ copy tất cả data vào func. lúc này data đó sẽ giống như local data của data, khi xử lý xong data đó sẽ được copi ngược lại data ban đầu. điều này gây lãng phí tài nguyên

## *Passing Arguments by Reference*


```
fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if (*a)[i] < 0 {
            (*a)[i] *= 2;
        }
    }
}
fn main() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    print!("{:?}", arr);
}

/*
[5, -8, 9, 0, -14, -2, 3, 5, 3, 1]
*/
```

kết quả lúc này đã chính xác, nhưng không cần copy data
lúc này xuất hiện 2 ký tự mới `*` và `&`. Những thứ này trong Rust có ý nghĩa giống với trong C, `&` mang ý nghĩa `địa chỉ của object trong RAM` *the (memory) address of the object* và `*` là `giá trị của object trong RAM` *the object that is present at the (memory) address*


## *Using References*

```
let a = 15;
let ref_a = &a;
print!("{} {} {} {}", a, *ref_a, ref_a, &ref_a);

/*
15 15 15 15
*/
```

```
let a = &&&7;
print!("{} {} {} {}", ***a, **a, *a, a);

/*
7 7 7 7
*/
```

## *Mutability of References*


```
let mut a: i32 = 10;
let mut b: i32 = 20;
let mut p: &mut i32 = &mut a;
println!("{} {}", *p, p);
*p += 1;
println!("{} {}", *p, p);
p = &mut b;
println!("{} {}", *p, p);
*p += 1;
println!("{} {}", *p, p);

/*
10 10
11 11
20 20
21 21
*/
```
