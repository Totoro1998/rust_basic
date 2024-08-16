// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("add_to_waitlist")
//         }
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// fn main() {
//     eat_at_restaurant()
// }

// // 注意 use 只能创建 use 所在的特定作用域内的短路径。
// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("add_to_waitlist")
//         }
//     }
// }

// use crate::front_of_house::hosting;

// // pub mod customer {
// //     pub fn eat_at_restaurant() {
// //         hosting::add_to_waitlist();
// //     }
// // }

// fn main() {
//     hosting::add_to_waitlist();
// }

// // 使用 as 关键字
// use std::fmt::Result;
// use std::io::Result as IoResult;

// 使用 pub use 重导出名称

// // 假设是在lib模式下，在这个修改之前，外部代码需要使用路径 p_21_modules_use::front_of_house::hosting::add_to_waitlist() 来调用 add_to_waitlist 函数。
// // 现在这个 pub use 从根模块重导出了 hosting 模块，外部代码现在可以使用路径 p_21_modules_use::hosting::add_to_waitlist。
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// 嵌套路径来消除大量的 use 行
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// // 通过 glob 运算符将所有的公有定义引入作用域
// use std::collections::*; //这个 use 语句将 std::collections 中定义的所有公有项引入当前作用域。
fn main() {
    println!("main")
}
