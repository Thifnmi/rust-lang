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

# **Defining Generic Functions and Structs**

## *Need of Generic Functions*

Rust kiểm tra kiểu dữ liệu nghiêm ngặt, vì vậy khi định nghĩa function cần xác định chính xác kiểu dữ liệu của parameters

```
fn square_root(x: f32) -> f32 {}
```

ví dụ khi gọi tới hàm `square_root` cần phải truyền chính xác kiểu dữ liệu như square_root(16.5f32) hay square_root(16.5f64 as f32) và không được truyền kiểu khác như square_root(16.5f64)

điều này gây bất tiện cho chính người viết functions cũng như người sử dụng gọi tới function đó. ví dụ nếu quyết định function đó sử dụng kiểu i16, nhưng mỗi khi gọi vào lại là kiểu i32 thì tốt nhất là nên thay đổi cách định nghĩa function


```
fn f(ch: char, num1: i16, num2: i16) -> i16 {
    if ch == 'a' {
        num1
    }
    else {
        num2
    }
}

print!("{}", f('a', 37, 41));

/*
37
*/
```

```
fn f(ch: char, num1: i16, num2: i16) -> i16 {
    if ch == 'a' {
        num1
    }
    else {
        num2
    }
}

print!("{}", f('a', 37.9, 41));

/*
error
*/

/*
convert data to i16
*/
print!("{}", f('a', 37.9 as i16, 41.5 as i16));
```

đoạn code trên sẽ báo lỗi và khi đó buộc phải convert về i16 và kết quả vẫn sẽ là 37 thay vì 37.9

nếu thay đổi cách định nghĩa function (thay thế i16 bằng f32 hoặc f64) thì các vấn đề được giải quyết. tuy nhiên mỗi khi call tới function phải sử dụng data dạng float

## *Defining and Using Generic Functions*

```
fn f1<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 } else { num2 }
}

// thay thế tham số `T` với kiểu dữ liệu i16
let a: i16 = f1::<i16>('a', 37, 41);

// thay thế tham số `T` với kiểu dữ liệu f64
let b: f64 = f1::<f64>('b', 37.2, 41.1);
println!("{} {}", a, b);

/*
37 41.1
*/
```

ở đây phía sau tên function có ký từ `T` ở phía trong ngoặc nhọn (angular brackets `<>`). đây là tham số kiểu của function

Điều đó có nghĩa những function được khai báo như thế không phải là function cụ thể mà là hàm chung được tham số hóa bởi tham số `T`. Hàm như vậy sẽ chỉ trở thành hàm cụ thể (concrete function) khi và chỉ khi biên dịch với tham số được định nghĩa cụ thể thay cho tham số `T`

Tham số T được định nghĩa và chỉ sử dụng được trong phạm vi của function.
với ví dụ bên trên nó được sử dụng 3 lần trong function và nó cũng có thể được sử dụng trong phần body của function chứ không phải bất kỳ nơi nào khác. trong khi tham số `ch` thuộc kiểu `char` thì cả tham số num1, num2 và giá trị của hàm đêù thuộc dạng chung `T`. khi sử dụng hàm như vậy cần phải thay thế tham số `T` bằng một kiểu cụ thể.

khi kiểu dữ liệu i16 được sử dụng điều đó có nghĩa là 2 số nguyên truyền vào phải có kiểu dữ liệu i16 và kết quả trả về của function cũng sẽ có kiểu i16
khi kiểu dữ liệu f64 được sử dụng điều đó có nghĩa là 2 số thập phân truyền vào phải có kiểu dữ liệu f64 và kết quả trả về của function cũng sẽ có kiểu f64
nếu tham số của function hay kiểu giá trị trả về hoặc cả 2 đều bị hoán đổi thì khi biên dịch sẽ xảy ra lỗi `mismatched types`

Như vậy khi code library/ các function dùng chung sẽ hạn chế các function vô nghĩa, chúng có thể viết application code sử dụng nó với 2 loại riêng biệt, các loại khác cũng có thể dễ dàng được xử dụng mà không cần phải thay đổi code library/ các function dùng chung

Trong `C` không cho phép những hàm dùng chung nhưng `C++` thì có và được gọi là function templates

## *Inferring the Parametric Types*

đoạn code phía trên chạy tốt nhưng có thể tối giản hơn được

```
fn f1<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 } else { num2 }
}

let a: i16 = f1('a', 37, 41);

let b: f64 = f1('b', 37.2, 41.1);
println!("{} {}", a, b);

/*
37 41.1
*/
```

các mệnh đề như `::<i16>` và `::<f64>` đã bị loại bỏ nhưng kết quả vẫn tương đương. đúng vậy, khi phân tích cú pháp biên dịch của hàm chung các giá trị được truyền vào sẽ được sử dụng để xác định kiểu của tham số
Như có vẻ như, các mệnh đề ::<i16> và ::<f64> đã bị xóa, dù sao cũng có được một chương trình tương đương. Thật vậy, trình biên dịch, khi phân tích cú pháp lệnh gọi hàm chung, sử dụng các loại giá trị được truyền làm đối số để xác định tham số loại.
Điều này có nghĩa là kiểu tham số được suy ra từ kiểu của các giá trị được sử dụng trong biểu thức chứa lệnh gọi hàm chung, đương nhiên các loại khác nhau được sử dụng phải nhất quán.

```
fn f<T>(a: T, _b: T) -> T { a }
let _a = f(12u8, 13u8);
let _b = f(12i64, 13i64);
let _c = f(12i16, 13u16);
let _d: i32 = f(12i16, 13i16);
```

ví dụ này chương trình sẽ gặp lỗi ở dòng lệnh `let _c = f(12i16, 13u16);` bởi 2 giá trị truyền vào có kiểu dữ liệu khác nhau và `let _d: i32 = f(12i16, 13i16);` vì kiểu dữ liệu trả về và biến được gán giá trị có kiểu khác nhau

nếu muốn tham số hóa một hàm có nhiều tham số có kiểu dữ liệu khác nhau hay chỉ định kiểu dữ liệu cho tham số đó


```
fn f<Param1, Param2>(_a: Param1, _b: Param2) {}
f('a', true);
f(12.56, "Hello");
f((3, 'a'), [5, 6, 7]);
```

## *Defining and Using Generic Structs*

```
struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}
let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };
```
Định nghĩa struct chung có tên là S nhận vào 2 tham số hóa là T1 và T2. Tham số thứ nhất được sử dụng 2 lần bởi 2 field trong struct trong khi tham số thứ 2 chỉ được sử dụng bởi 1 field

```
struct TS<T1, T2>(char, T1, T1, T2);
let _se = TS('a', 34, 782, 0.02);
```
tương tự như phía trên nhưng thay vì struct thì ví dụ này là tuple-struct

Ngoài ra cũng có thể biểu diễn việc tham số hóa cho struct rõ ràng hơn

```
struct S<T1, T2> { c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}
let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };

struct TS<T1, T2> (char, T1, T1, T2);
let _ts = TS::<u16, f32> ('a', 34, 782, 0.02);
```

structs chung (generic structs) không được cho phép trong `C` nhưng trong `C++` thì được. chúng được gọi là `class templates` và `the struct templates`.


## *Genericity Mechanics*

để hiểu rõ hơn về cách thức hoạt động hãy thử đặt mình vào vị trí của trình biên dịch để theo dõi quá trình biên dịch. về mặt khái niệm quá trình biên dịch được chia làm nhiều giai đoạn. hãy thử sức với đoạn code này 

```
fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
let x = swap(3i16, 4u16);
let y = swap(5f32, true);
print!("{:?} {:?}", x, y);
```

giai đoạn đầu tiên chỉ scan source code, mỗi khi tìm thấy hàm chung (generic function) nó sẽ load qua cấu trúc dữ liệu của nó bên trong cả function, ở giai đoạn này chỉ kiểm tra lỗi cú pháp
giai đoạn thứ 2 source code được scan lại một lần nữa, mỗi khi biên dịch gặp lệnh gọi hàm chung (generic function) ...

## *Generic Arrays and Vectors*

Ngay từ đầu array và vector đã là những dạng chung, trong thực tế array là 1 phần của Rust còn vector là cấu trúc được xác định trong các thư viên của Rust

## *Generic Enums*

Enum cũng có thể có dạng chung

```
enum Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode, char),
    Uncertainty,
}
fn main() {
    let mut _res = Result1::Success::<u32, u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u16, 'd');
}
```

generic enum được sử dụng nhiều trong các thư viện của Rust
một trong số những vấn đề tiêu biểu dùng enum để giải quyết trong Rust đó là nếu 1 function có thể bị lỗi thì nó phải làm gì?

ví dụ hàm pop() để loại bỏ item cuối cùng của vector và trả về item nếu tồn tại. nhưng nếu `vec![0; 0].pop()` (xóa item khỏi vector rỗng) thì làm gì?
một vài ngôn ngữ thì không xác định hành vi này, nó có thể dẫn tới sự cố hoặc kết quả không đoán trước được. Nhưng trong Rust thì tránh càng nhiều hành vi không được xác định trước thì càng tốt
một số ngôn ngữ thì thì có `exception`, khi đó các hành vi không xác định sẽ được xử lý bởi đoạn code trong block hoặc bởi lời gọi hàm hiện tại và cũng có thể không đuộc xử lý dẫn tới sự cố. Trong Rust thì không có khái niệm exception.
một vài ngôn ngữ khác thì trả về giá trị `null` nhưng vector có thể chứa hầu hết mọi loại kiểu giá trị và trong đó cũng có nhiều loại không có giá trị `null`

Vậy trong Rust xử lý như nào?

```
let mut v = vec![11, 22, 33]; for _ in 0..5 {
    let item: Option<i32> = v.pop();
    match item {
        Some(number) => print!("{}, ", number),
        None => print!("#, "),
    }
}

/*
33, 22, 11, #, #,
*/
```

v là vector có 3 giá trị
vòng for sẽ lặp lại 5 lần, mỗi lần sẽ remove 1 item trong vector v, nếu thành công sẽ in ra giá trị của item, nếu thất bại in ra `#`

với `Option` là generic enum được định nghĩa bởi thư viện tiêu chuẩn của Rust

```
enum Option<T> {
    Some(T),
    None,
}
```


## *Error Handling*

Trong thư viện của Rust cũng đã có 1 số generic enum để handle trường hợp function không trả về được đúng giá kiểu giá trị mong muốn

```
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { Err(format!("Divide by zero")) } else { Ok(numerator / denominator) }
}
fn main() {
    print!("{:?}, {:?}", divide(8.0, 2.0), divide(8.0, 0.0));
}

/*
Ok(4.0), Err("Divide by zero")
*/

```

Tương tự `Result` cũng giống như `Option` đều là generic enum trong thư viện tiêu chuẩn của Rust. tuy nhiên `Option` không có giá trị nào được thể hiện khi không có kết quả còn `Result` có thể thêm mô tả khi thực hiện gặp lỗi

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

ở ví dụ trên T có dạng f64 vì vậy kết quả của function phải là f64 và E là String vì chỉ muốn hiển thị message. các kết quả trên được in ra dưới dạng thông tin debug. tuy nhiên trong production điều đó là không được chấp nhận

```
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { Err(format!("Divide by zero")) } else { Ok(numerator / denominator) }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}

fn main() {
    show_divide(8.0, 2.0);
    show_divide(8.0, 0.0);
}

/*
8/2= 4
Cannot divide 8 by 0: Divide by zero
*/
```

## *Enum Standard Utility Functions*

`Result` và `Option` là 2 thư viện tiêu chuẩn cho phép nắm bắt linh hoạt và hiệu quả tất cả các trường hợp có thể xảy ra trong quá trình code thực tế. tuy nhiên sử dụng `match` để so sánh gặp khá nhiều bất tiện. vì vậy trong thư viện tiêu chuẩn có một vài function để giải mã giá trị của `Result` và `Option`


```
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { Err(format!("Divide by zero")) } else { Ok(numerator / denominator) }
}

fn main() {
    let r1 = divide(8.0, 2.0);
    let r2 = divide(8.0, 0.0);
    println!("{} {}", r1.is_ok(), r2.is_ok());
    println!("{} {}", r1.is_err(), r2.is_err());
    println!("r1 value: {}", r1.unwrap());
    println!("r2 value: {}", r2.unwrap());
}

/*
true false
false true
r1 value: 4
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Divide by zero"'
*/
```

`is_ok()` trả về `true` nếu `Ok()` của `Result` được áp dụng và ngược lại `is_er()` trả về `true` nếu `Err()` của `Result` được áp dụng. điều này có nghĩa là `is_err` và `! is_ok` trả về kết quả giống nhau
`unwrap()` trả về giá trị của biến khi `Ok()` của `Result` được áp dụng
`unwrap()` cũng là 1 function của Option enum. khi đó có thể lấy ra giá trị của cả vector như sau 

```
let mut v = vec![11, 22, 33];
for _ in 0..v.len() {
    print!("{}, ", v.pop().unwrap())
}

/*
33, 22, 11, 
*/
```

`unwrap()` được sử dụng nhiều trong Rust bỏi nó nhanh và gọn

# **Allocating Memory**

## *The Various Kinds of Allocation*

để hiểu rõ hơn về Rust cũng như các ngôn ngữ lập trình hệ thống khác ví dụ như `C`, cần phải hiểu rõ các khái niệm khác nhau về  `memory allocation`, `static allocation`, `stack allocation`, `heap allocation`


## *Linear Addressing*

trong bất kỳ phần cứng máy tính nào đều có 1 thứ dùng để đọc và ghi nó được gọi là RAM, bao gồm các chuỗi bytes dài và có thể truy cập.  bytes đầu tiên trong RAM có vị trí là 0 trong khi byte cuối cùng có vị trí là kích thước bộ nhớ -1

nói một cách đơn giản thì hiện tại chúng ta có 2 loại máy tính
    - những thứ tại 1 thời điểm chỉ có thể chạy 1 tiến trình và tiến trình đó sử dụng trực tiếp bộ nhớ vật lý được gọi là `hệ thống bộ nhớ thưc (real-memory systems)`
    - những hệ thống có thể chạy được đa chương trình, cung cấp không gian địa chỉ ảo cho mỗi chương trình chạy được gọi là `hệ thống bộ nhớ ảo (vitual-memory systems)`

...

## *Static Allocation*

```
static _A: u32 = 3;
static _B: i32 = -1_000_000;
static _C: f64 = 5.7e10;
static _D: u8 = 200;
```

`static` giống như `let`, cả 2 đều dùng để khởi tạo biến và tùy ý khởi tạo. tuy nhiên vẫn có những điểm khác biêt:
    - `static` sử dụng `static allocation` trong khi `let` sử dụng `stack allocation`
    - `static` yêu cầu kiểu dữ liệu của biến, trong khi `let` thì không với `let` đó là 1 option
    - code thông thường không thể thay đổi giá trị của biến `static` ngay cả khi cả khi có `mut`. điều này vì lý do an toàn, do vậy các biến `static` trong Rust thường không thể thay đổi
    - qui tắc đặt tên biến static: chỉ chứa các ký tự viết hoa và ngăn cách các từ bằng underscore `_`, nếu vi phạm khi biên dịch sẽ có cảnh báo được đưa ra

1 nhược điểm của `static allocation` là không thể tạo được hàm đệ quy (các hàm trực tiếp hoặc gián tiếp gọi chúng)
1 nhược điểm nữa là nếu tất cả các biến trong chương trình đều là `static allocation`, khi thực thi nếu chương trình nhiều biến mỗi lần chỉ sử dụng 1 phần nhỏ các biến sẽ gây tốn nhiều bộ nhớ
thêm vào đó biến static không an toàn khi thay đổi
vì vậy trong Rust chúng không được sử dụng nhiều
tuy nhiên `static allocation` được sử dụng nhiều cho 2 kiểu dữ liệu: file thực thi nhị phân (executable binary code thực ra nó cũng không thực sự là dữ liệu) và các chuỗi ký tự 

## *Stack Allocation*

vì những thiếu sót của static allocation, Rust phân bổ các object vào stack mỗi khi biến được khai báo với từ khóa `let` và cả khi truyền tham số vào hàm được gọi. cái được gọi là `stack` là một không không thể thiếu của mọi không gian địa chỉ

trong thực tế có 1 stack cho mỗi luồng chứ không chỉ là stack cho mỗi tiến trình. nếu hệ thống hỗ trợ đa luồng, mỗi khi chạy 1 chương trình tức là mỗi tiến trình được tạo sẽ có 1 luông được tạo theo và khỏi chạy bên trong tiến trình đó. sau đó trong tiến trình các luồng khác có thể được khởi tạo và chạy theo. mỗi khi luồng được tạo ra, bao gồm luồng chính của mỗi tiến trình hệ điều hành sẽ được yêu cầu để phân bổ một phần địa chỉ không gian. đó chính là `stack` của luồng. trong hệ thống bộ nhớ thực chỉ có 1 `stack` được tạo ra khi chương trình khởi chạy

mỗi luồng lưu trữ địa chỉ các đầu của `stack` đó. thông thường phần cuối giá trị cao hơn được gọi là phần đáy cảu ngăn xếp, phần cuối có giá trị nhỏ hơn được coi là phần trên cùng của ngăn xếp


```
let _a: u32 = 3;
let _b: i32 = -1_000_000;
let _c: f64 = 5.7e10;
let _d: u8 = 200;
```

...

## *Limitations of Stack Allocation*

`stack allocation` rất tiện lợi nhưng lại có vài hạn chế
    - kích thước. kích thước phụ thuộc vào hệ điều hành và có thể bị giảm với 1 số ứng dụng nhưng theo thứ tự độ lớn nó rơi vào khoảng vài megabytes
    - Rust cho phép phân bổ stack chỉ với các object có kích thước biết trước khi biên dịch như kiểu và mảng nguyên thủy, không cho phân bổ stack các đối tượng có kích thước chỉ được xác định khi chạy như vector
    - không được phân bổ/giải phóng các đối tượng explicitly (rõ ràng?) trong ngăn xếp, mọi biến đều đươc tự động phân bổ khi hàm mà nó khai báo được gọi, ngay cả khi nó được khai báo trong  của hàm đó và nó sẽ bị hủy cấp phát khi quá trình thực thi hàm đó kết thúc. không thể ghi đè hành vi đó.


...

## *Heap Allocation*


...


## *Heap Management*

...

## *The Behavior of Box*

...

## *Similarity with C and C++*

...


## *Boxing and Unboxing*

...

## *Register Allocation*

...



# **Data Implementation**

## *Discovering the Size of Objects*

từ source file, Rust có thể biên dịch ra tất cả ngôn ngữ máy, miễn là nó hoạt động theo cách mà Rust chỉ định cho mã nguồn đó
Do đó, với một biến ta không xác định được nó sử dụng bao nhiêu bit bộ nhớ và vị trí của nó trong bộ nhớ. Nhưng trình biên dịch thậm chí có thể xóa biến đó khỏi bộ nhớ vì nó không bao giờ được sử dụng hoặc vì nó được lưu trong thanh ghi bộ xử lý.
Tuy nhiên, nên xem cách triển khai điển hình có thể có của việc sắp xếp dữ liệu được sử dụng bởi chương trình Rust.

```
print!("{} ", std::mem::size_of::<i32>());
print!("{} ", std::mem::size_of_val(&12));

/*
4 4
*/
```
trong câu lệnh đầu tiên trình biên dịch sử dụng thư viện module `std` (standard) sau đó có thêm sub-module `mem` (memory) cuối cùng là generic function `size_of`
trình biêm dịch cụ thể hóa generic function bằng cách sử dụng kiểu i32 sau đó tạo ra lệnh gọi hàm cụ thể mà không truyền bất cứ tham số nào. Hàm như vậy sẽ trả về số byte (chính xác là “octet của bit”) được chiếm bởi bất kỳ đối tượng nào thuộc loại đã chỉ định. Thông thường, việc gọi hàm như vậy sẽ được nội tuyến và do đó code được tạo ra chỉ là một số không đổi. Trên thực tế, một số 32 bit xuất hiện chiếm 4 byte.
Lưu ý rằng có thể gọi hàm này ngay cả khi trong chương trình không có đối tượng thuộc loại được chỉ định.

ở câu lệnh thứ 2 tương tự câu thứ nhất nhưng gọi generic function khác là `size_of_val` “size of value”. Trong trường hợp này, loại tham số cần thiết để cụ thể hóa hàm chung được suy ra từ đối số và do đó không cần phải chỉ định rõ ràng. Thay vào đó, trong câu lệnh đầu tiên không có đối số và do đó tham số kiểu là bắt buộc.
Khi hàm cụ thể size_of_val được gọi, một tham chiếu bất biến tới một đối tượng sẽ được truyền tới nó. Hàm trả về kích thước tính bằng byte của đối tượng đó.


## *The use Directive*

trong Rust `use` giống như trong `C++`



```
use std::mem;

fn main() {
    println!("{} ", std::mem::size_of::<i32>());
    println!("{} ", std::mem::size_of_val(&12));
}

use std::mem::size_of;
use std::mem::size_of_val;
fn main() {
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}

use std::mem::*;
fn main() {
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}
```

dấu * `the asterisk` là ký tự đại diện để import tất cả các submodule

## *The Sizes of the Primitive Types*

```
use std::mem::*;
println!(
    "{} {} {} {} {} {} {} {} {} {} {} {}",
    size_of::<i8>(),
    size_of::<u8>(),
    size_of::<i16>(),
    size_of::<u16>(),
    size_of::<i32>(),
    size_of::<u32>(),
    size_of::<i64>(),
    size_of::<u64>(),
    size_of::<f32>(),
    size_of::<f64>(),
    size_of::<bool>(),
    size_of::<char>()
);
/*
1 1 2 2 4 4 8 8 4 8 1 4
*/
```


với 1 vài kiểu dữ liệu khác sẽ phụ thuộc vào nên tảng của thiết bị đang biên dịch

```
use std::mem::*;
println!(
    "{} {} {} {}",
    size_of::<isize>(),
    size_of::<usize>(),
    size_of::<&i8>(),
    size_of::<&u32>()
);

/*
In a 64-bit system 8 8 8 8
while in a 32-bit system 4 4 4 4
*/
```

2 giá trị được in ra cuối cùng là giá trị tham chiếu, đối tượng được tham chiếu (pointer) có kích thước bằng một địa chỉ bộ nhớ


## *The Representation of Primitive Types*

trong Rust không khuyến khích việc truy cập vào biểu diễn bên trong các đối tượng cũng bởi vì việc này không dễ thực hiện nhưng vẫn có mẹo để làm điều đó

```
fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(o as *const _ as *const u8, std::mem::size_of::<T>()) }
}

fn main() {
    println!("{:?}", as_bytes(&1i8));
    println!("{:?}", as_bytes(&2i16));
    println!("{:?}", as_bytes(&3i32));
    println!("{:?}", as_bytes(&(4i64 + 5 * 256 + 6 * 256 * 256)));
    println!("{:?}", as_bytes(&'A'));
    println!("{:?}", as_bytes(&true));
    println!("{:?}", as_bytes(&&1i8));
}

/*
[1]
[2, 0]
[3, 0, 0, 0]
[4, 5, 6, 0, 0, 0, 0, 0]
[65, 0, 0, 0]
[1]
[125, 159, 5, 4, 1, 0, 0, 0]
*/
```

hàm chung as_bytes ở đây đơn giản là lấy tham chiếu đến một đối số thuộc bất kỳ loại dữ liệu nào và trả về một object đại diện cho chuỗi bute chưa trong object như vậy
bằng cách in ra như vậy ta có thể thấy cách biểu diễn của bất kỳ đối tượng nào dưới dạng chuỗi byte lưu trong RAM

giải thích cho ví dụ bên trên:
đầu tiên i8 có giá trị 1 được lưu trữ tring một byte đơn
sau đó i16 có giá trị 2 được lưu trữ dưới dạng một cặp byte, trong đó byte thứ nhất là 2 và byte thứ 2 là 0. điều này xảy ra trên cả hệ thống 32 bit và 64 bit. nhưng chỉ trong cái gọi là `little-endian` hardware architectures. Thay vào đó, với `big-endian` sẽ in ra [0,2]

các dong tiếp theo cũng tương tự như vậy

lưu ý:
kiểu char được lưu dưới dạng 32 bit chứ các giá trị unicode của ký tưh đó còn bool được lưu dưới dạng 1 byte 1 là đúng, 0 là sai

dòng cuối cùng sẽ in ra địa chỉ của số i8. địa chỉ như vậy chiếm 8 byte đối với bộ xử lý 64 bit và khác nhau tùy vào từng lần chạy

## *Location of Bytes in Memory*

với bộ nhớ ảo cũng có thể xem được địa chỉ của đối tượng

```
let b1 = true;
let b2 = true;
let b3 = false;
print!(
    "{} {} {}",
    &b1 as *const bool as usize, &b2 as *const bool as usize, &b3 as *const bool as usize
);

/*
64-bit system, 140727116566237 140727116566238 14072711656623

32-bit system, in ra được 3 số nhỏ hơn 5 tỷ
*/
```

## *Sizes of Composite Data Types*

```
use std::mem::*;

enum E1 {
    E1a,
    E1b,
};
enum E2 {
    E2a,
    E2b(f64),
};
println!(
    "{} {} {} {} {} {}",
    size_of_val(&[0i16; 80]),
    size_of_val(&(0i16, 0i64)),
    size_of_val(&[(0i16, 0i64); 100]),
    size_of_val(&E1::E1a),
    size_of_val(&E2::E2a),
    size_of_val(&vec![(0i16, 0i64); 100])
);

/*
160 16 1600 1 16 24
*/
```

    - mảng gồm 80 số 16-bit chiếm 160 byte, tức là 80 * 2 do đó không bị lãng phí
    - tuple gồm 1 số 16 byte và 1 số 64 byte chiếm 16 byte giống như nếu cả 2 số đều chiếm 8 byte do đó phần đệm 6 byte được thêm vào
    - mảng gồm 100 tuple 16 bit chiếm 1600 byte do đó không có khoảng đệm giữa các phần tử của mảng nhưng phần đệm của mỗi mục được nhân với độ dài của mảng
    - enum tất cả các biến không có field dữ liệu luôn chỉ chiếm 1 byte
    - enum có biến lớn nhất chứa 8 byte chiếm 16 byte ngay cả khi giá trị hiện tại không có dữ liệu, vì có 7 byte đệm
    - vector gồm 100 tuple có kiểu dữ liệu byte chiếm 24 byte nhưng tất nhiên vẫn thiếu gì đó trong thước đo này

## *Vector Allocation*

vector được triển khai dưới dạng cấu trúc 2 đối tượng: header có kích thước cố định được phân bổ theo stack và phần đệm biến dài thay đổi được theo heap-allocation
về mặt lý thuêys có 1 vài khác để triển khai vector data structure

...


```
let mut v = vec![0; 0];
println!("{} {}", v.len(), v.capacity());
v.push(11);
println!("{} {}", v.len(), v.capacity());
v.push(22);
println!("{} {}", v.len(), v.capacity());
v.push(33);
println!("{} {}", v.len(), v.capacity());
v.push(44);
println!("{} {}", v.len(), v.capacity());
v.push(55);
println!("{} {}", v.len(), v.capacity());


/*
0 0
1 4
2 4
3 4
4 4
5 8
*/
```


```
let mut v = vec![0; 0];
let mut prev_capacity = std::usize::MAX;
for i in 0..1_000 {
    let cap = v.capacity();
    if cap != prev_capacity {
        println!("{} {} {}", i, v.len(), cap); prev_capacity = cap;
    }
    v.push(1);
}


/*
0 0 0
1 1 4
5 5 8
9 9 16
17 17 32
33 33 64
65 65 128
129 129 256
257 257 512
513 513 1024
*/
```


# **Defining Closures**

## *The Need for “Disposable” Functions*


```
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort();
print!("{:?}", arr);


/*
[0, 1, 4, 7, 8, 10, 12, 45]
*/
```

đoạn code trên sắp xếp array theo thứ tự tăng dần (ascending), tuy nhiên nếu muốn theo thứ tự giảm dần (descending) hay sử dụng tiêu chí khác thì sao? không có prepacket function nào cả, phải tự gọi hàm `sort_by` và truyền tham chiếu tới hàm so sánh. hàm nhận vào 2 số và sẽ trả dấu hiệu xem số nào đứng trước


```
use std::cmp::Ordering;
fn desc(a: &i32, b: &i32) -> Ordering {
    if a < b {
        Ordering::Greater
    } else if a > b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(desc);
    println!("{:?}", arr);
}

/*
[45, 12, 10, 8, 7, 4, 1, 0]
*/
```

cách này có hoạt động tuy nhiên nó có 1 vài nhược điểm
    - func `desc` được tạo ra và chỉ sử dụng duy nhất 1 lần trong cả quá trình. thông thường function được tạo ra không chỉ để sử dụng 1 lần, hơn thế, phần thân func được tạo ra có thể được mở rộng. tuy nhiên function `sort_by` yêu cầu truyền vào 1 function. vậy điều cần thiết ở đây là một `inline anonymous function` (hàm ẩn danh nội tuyến?), một hàm được khai báo cùng lúc và có thể sử dụng được
    - ngoài ra, có kiểu tùy chọn là khai báo variable nhưng nó bắt buộc với các tham số và giá trị trả về khi khai báo function

...

## *Capturing the Environment*

các hàm trong Rust không thể truy cập tới biến khai báo bên ngoài chúng, có thể truy cập tới `static` nhưng không thể truy cập tới các biến `stack allocation` được khai báo với `let`

```
fn main() {
    let two = 2.;
    print_double(17.2);
    fn print_double(x: f64) {
        print!("{}", x * two);
    }
}

/*
error[E0434]: can't capture dynamic environment in a fn item
  |
  |         print!("{}", x * two);
  |                          ^^^
  |
  = help: use the `|| { ... }` closure form instead
*/
```

vì vậy có thể sửa thành

```
const TWO: f64 = 2.;
fn print_double(x: f64) {
    print!("{}", x * TWO);
}
print_double(17.2);

hoặc

static TWO: f64 = 2.;
fn print_double(x: f64) {
    print!("{}", x * TWO);
}
print_double(17.2);
```

việc hạn chế như vậy có lý do chính đáng: các biến bên ngoài đi vào function hiệu quả, tuy nhiên lại không rõ ràng ở trong func gây việc ra hiểu lầm trong khi đọc hiểu code
nhưng khi 1 hàm chỉ có thể được gọi ở ở nơi có đã được định nghĩa, thì việc nó truy cập vào các biến bên ngoài không khiến function trở nên khó hiểu hơn, bởi vì các biến bên ngoài đó đã có sẵn trong câu lệnh khai báo.
Do đó, các yêu cầu đối với tính năng: an inline anonymous function, with type inference, a single expression as body the capture of any valid variable.

## *Closures*

vì tính hữu dụng rất lớn nên trong Rust có một tính năng có tên là `closure`. nó là 1 hàm tiện dụng hơn để định nghĩa small anonymous functions, và gọi tới chúng ngay tại nơi định nghĩa

trong thực tế, bạn cũng có thể định nghĩa `closure` gắn vào biến, đặt tên và gọi tới nó bằng tên biến. tuy nhiên đó không phải cách sử dụng điển hình nhất của `closure`

```
use std::cmp::Ordering;
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    arr.sort_by(desc);
    print!("{:?}", arr);
}

/*
[45, 12, 10, 8, 7, 4, 1, 0]
*/
```

điểm khác nhau với ví dụ trước đó:
    - fn được thay bằng `let`
    - ký tự `=` được thêm vào sau khi khai báo biến
    - `(` và `)` đưuojc loại bỏ và thay vào đó là `|`
    - `;` được thêm vào ở cuối


```
use std::cmp::Ordering;
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort_by(|a, b| {
    if a < b {
        Ordering::Greater
    } else if a > b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
});
print!("{:?}", arr);
```



```
arr.sort();
arr.sort_by(|a, b| a.cmp(b));
```

```
arr.sort_by(|a, b| (&-*a).cmp(&-*b));
arr.sort_by(|a, b| b.cmp(a));
```

```
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort_by(|a, b| b.cmp(a));
print!("{:?}", arr);
```

## *Other Examples*
```
let factor = 2;
let multiply = |a| a * factor;
print!("{}", multiply(13));
let multiply_ref: &(Fn(i32) -> i32) = &multiply; print!(
    " {} {} {} {} {}",
    (*multiply_ref)(13),
    multiply_ref(13),
    (|a| a * factor)(13),
    (|a: i32| a * factor)(13),
    |a| -> i32 { a * factor }(13));
```


```
print!(
    "{}",
    (|v: &Vec<i32>| {
        let mut sum = 0;
        for i in 0..v.len() {
        sum += v[i];
        }
        sum
    })(&vec![11, 22, 34]));
```