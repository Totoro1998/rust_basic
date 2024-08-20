// fn main() {
//     panic!("crash and burn");
// }

// // 可以使用 RUST_BACKTRACE=1 cargo run 来得到一个 backtrace。backtrace 是一个执行到目前位置所有被调用的函数的列表。
// // Rust 的 backtrace 跟其他语言中的一样：阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。
// // 这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。
// fn main() {
//     let v = vec![1, 2, 3];
//     v[99];
// }

use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

// fn main() {
//     let greeting_file_result = File::open("hello1.txt");
//     // let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }

// // 匹配不同的错误
// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         },
//     };
// }

// // 失败时 panic 的简写：unwrap 和 expect
// fn main() {
//     // let greeting_file = File::open("hello1.txt").unwrap();
//     // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
//     let greeting_file =
//         File::open("hello.txt").expect("hello.txt should be included in this project");
// }

// // 传播错误
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// 使用?运算符
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// // ?运算符链式调用
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

// // 更简便的写法
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let username_result = read_username_from_file();
//     let username = match username_result {
//         Ok(username) => username,
//         Err(e) => panic!("Problem with the file: {e:?}"),
//     };
//     println!("{username}")
// }

// // ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。
// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }

// 可以将 Box<dyn Error> 理解为 “任何类型的错误”。
// 当 main 函数返回 Result<(), E>，如果 main 返回 Ok(()) 可执行程序会以 0 值退出，而如果 main 返回 Err 值则会以非零值退出；
// 成功退出的程序会返回整数 0，运行错误的程序会返回非 0 的整数。
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// ? 也可用于 Option<T> 值。如同对 Result 使用 ? 一样，只能在返回 Option 的函数中对 Option 使用 ?。
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // let text = String::from("\nhi");
    let text = String::from("hi");
    let last_char_options = last_char_of_first_line(&text);
    let last_char = match last_char_options {
        Some(value) => value,
        None => panic!("no value"),
    };
    println!("{last_char}")
}
