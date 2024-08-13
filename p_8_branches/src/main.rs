// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// 代码中的条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。
// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// 使用 else if 处理多重条件
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// 在 let 语句中使用 if
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

//if 的每个分支的可能的返回值都必须是相同类型
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
