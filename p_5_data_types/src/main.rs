// use std::io;

// // 整型
// fn main() {
//     let value_i8: u8 = 8; // 无符号
//     let value_u8: i8 = -8; // 有符号
//     println!("{value_i8},{value_u8}")
// }

// // 浮点型
// fn main() {
//     let value_f64 = 2.0; // 默认为f64

//     let value_f32: f32 = 3.0; // f32

//     println!("{value_f64},{value_f32}")
// }

// //数值运算
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // 结果为 -1

//     // remainder
//     let remainder = 43 % 5;
//     println!(
//         "sum={},difference={},product={},quotient={},truncated={},remainder={}",
//         sum, difference, product, quotient, truncated, remainder
//     )
// }

// // 布尔类型
// fn main() {
//     let value_t = true;
//     let value_f: bool = false; // with explicit type annotation

//     println!("{value_t},{value_f}")
// }

// // 字符类型
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';

//     println!("{c},{z},{heart_eyed_cat}")
// }

// // 元祖类型
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     let first_value = tup.0;

//     println!("x={},y={},z={},first_value={}", x, y, z, first_value);
// }

// // 数组类型
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
//     println!("first={},second={}", first, second);
// }

// // 无效的数组元素访问
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// // 字符串字面量
// fn main() {
//     // let speech = "\"Ouch!\" said the well.\n"; // "Ouch!" said the well.
//     //     let speech = "In the room the women come and go,
//     //  Singing of Mount Abora";
//     //     let speech = "It was a bright, cold day in April, and \
//     //  there were four of us—\
//     //  more or less.";

//     // let speech = r"C:\Program Files\Gorillas";
//     // let speech = r"这是一个原始字符串，包含反斜杠：\ 和引号：\";
//     // let speech = r#"""""这是一""""个原始字符串，包含反斜杠：\ 和引号：\""""""#;
//     // let speech = r##"""""这是一""""#个原始字符串，包含反斜杠：\ 和引号：\""""""##;
//     let speech = r###"""""这是一""""##个原始字符串，包含反斜杠：\ 和引号：\""""""###;
//     println!("{speech}");
// }

// // 字节串
// fn main() {
//     let method = b"GET";
//     assert_eq!(method, &[b'G', b'E', b'T']); // true
// }

fn main() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";
    println!("{}", noodles.len()) // 7
}
