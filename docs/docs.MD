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

# **data types:**

*string*

```
let x = "my string"
```

*integer*

```
let number = 12345
```

*float*

```
let f = 12.3
let f = 12.
```

*bool*

```
let b = true
let b = false
```

*array*

```
let my_array = ["my", "array"]
```

*mutable array* (array in array)

```
let mut my_mutable_array = ["my", "array"];
my_mutable_array[0] = ["this is"];
print!("{} {}.", my_mutable_array[0], my_mutable_array[1]);

/*
result:
this is array.
*/
```

*Multidimensional Arrays* (Mảng đa chiều)

```
let multidimensional_arrays = [[[[]]]]
```

*Vector*

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

**expression**

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

conditioned loop

```
while condition {
    // logic
}
```

infinite loop

```
while true {
    // logic
}
```

couting loop (for)

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

Arrays of Specified Size

```
let `<mut> `_arr = [`<default value>`; `<length>`];
```

Multidimensional Arrays

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

*Non-Decimal Numeric Bases*

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

*Underscore in Numeric Literals (_)

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

**Number**

*Signed Integer Numbers* // Số nguyên có dấu

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

*Unsigned Integer Numbers* // Số nguyên không dấu

Giống như signed integer, unsigned integer cũng có các dạng 8-bit unsigned integer, 16-bit unsigned integer, 32-bit unsigned integer, 64-bit unsigned integer
u8 cũng cho phép biểu diễn 256 giá trị giống như i8 tuy nhiên chỉ cho phép giá trị nguyên dương và trong khoảng từ 0 -> 255

Mục đích của các loại số nguyên phụ thuộc vào mục đích sử dụng

• on 16-bit computers, it is an unsigned 16-bit integer;
• on 32-bit computers, it is an unsigned 32-bit integer;
• on 64-bit computers, it is an unsigned 64-bit integer.

index của array hay vector phải là kiểu dữ liệu unsigned integer vì nó giống với địa chỉ giá trị trong RAM và cũng chỉ có kiểu dữ liệu này được phép sử dụng

Rust không hỗ trợ 16-bit system nhưng lại hỗ trợ cả 32-bit system và 64-bit system

Việc lựa chọn 32-bit hay 64-bit không liên quan tới việc tối ưu code mà nó phụ thuộc vào việc sau khi biên dịch sẽ chạy trên hệ thống nào? (32-bit hay 64-bit). Với mục đích đó, Rust có thêm 2 loại là isize và usize

*Float*
Trong rust có 2 kiểu float là f32 và f64 tương đương với 32-bit floating-point numbers và 64-bit floating-point numbers
f32 hiển thị 7 số phần thập phân, f64 hiển thị 16 số phần thập phân

**Explicit Conversions**


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

*Type suffixes of numeric literals*

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

**All the Numeric Types**

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


**Booleans and Characters**

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