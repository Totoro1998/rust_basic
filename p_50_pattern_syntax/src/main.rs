// // 匹配字面值
// fn main() {
//     let x = 1;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// // 匹配命名变量
// fn main() {
//     let x = 1;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// // 多个模式
// fn main() {
//     let x = 1;

//     match x {
//         1 | 2 => println!("one or two"), // one or two
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// // 通过..=匹配值的范围
// fn main() {
//     let x = 5;

//     match x {
//         1..=5 => println!("one through five"),
//         _ => println!("something else"),
//     }
// }

// // 范围只允许用于数字或 char 值。
// fn main() {
//     let x = 'c';

//     match x {
//         'a'..='j' => println!("early ASCII letter"), // early ASCII letter
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
// }

// // 解构结构体
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point { x: 0, y: 7 };

//     let Point { x: a, y: b } = p;
//     let Point { x: c, .. } = p;

//     assert_eq!(0, a);
//     assert_eq!(0, c);
//     assert_eq!(7, b);
// }

// // 简写形式
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point { x: 0, y: 7 };

//     let Point { x, y } = p;
//     assert_eq!(0, x);
//     assert_eq!(7, y);
// }

// // 可以使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let p = Point { x: 0, y: 7 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {x}"),
//         Point { x: 0, y } => println!("On the y axis at {y}"), // On the y axis at 7
//         Point { x, y } => {
//             println!("On neither axis: ({x}, {y})");
//         }
//     }
// }

// // 解构枚举
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);

//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.");
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {x} and in the y direction {y}");
//         }
//         Message::Write(text) => {
//             println!("Text message: {text}");
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {r}, green {g}, and blue {b}")
//         }
//     }
// }

// // 解构嵌套的结构体和枚举
// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change color to red {r}, green {g}, and blue {b}");
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change color to hue {h}, saturation {s}, value {v}")
//         }
//         _ => (),
//     }
// }

// // 解构结构体和元组
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
//     println!("{feet}")
// }

// // 使用_忽略整个值
// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {y}");
// }

// fn main() {
//     foo(3, 4);
// }

// // 使用嵌套的_忽略部分值
// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }

//     println!("setting is {setting_value:?}");
// }

// // 可以在一个模式中的多处使用下划线来忽略特定值
// fn main() {
//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (first, _, third, _, fifth) => {
//             println!("Some numbers: {first}, {third}, {fifth}")
//         }
//     }
// }

// // 通过在名字前以一个_开头来忽略未使用的变量
// fn main() {
//     let _x = 5;
//     let y = 10;
// }

// // _x 仍会将值绑定到变量，而 _ 则完全不会绑定
// fn main() {
//     let s = Some(String::from("Hello!"));

//     // if let Some(_s) = s {
//     //     println!("found a string"); // borrow of partially moved value: `s`
//     // }

//     if let Some(_) = s {
//         println!("found a string");
//     }

//     println!("{s:?}"); // borrow of partially moved value: `s`
// }

// // 用..忽略剩余值
// fn main() {
//     struct Point {
//         x: i32,
//         y: i32,
//         z: i32,
//     }

//     let origin = Point { x: 0, y: 0, z: 0 };

//     match origin {
//         Point { x, .. } => println!("x is {x}"),
//     }
// }

// //  用..忽略剩余值
// fn main() {
//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (first, .., last) => {
//             println!("Some numbers: {first}, {last}");
//         }
//     }
// }

// // 匹配守卫提供的额外条件
// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x % 2 == 0 => println!("The number {x} is even"), // The number 4 is even
//         Some(x) => println!("The number {x} is odd"),
//         None => (),
//     }
// }

// // 使用匹配守卫来解决模式中变量覆盖的问题
// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n = {n}"),
//         _ => println!("Default case, x = {x:?}"), // Default case, x = Some(5)
//     }

//     println!("at the end: x = {x:?}, y = {y}");
// }

// // 匹配守卫中使用 或 运算符 | 来指定多个模式
// fn main() {
//     let x = 4;
//     let y = true;

//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }

// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
