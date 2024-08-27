// // 基本语法
// fn main() {
//     let b = Box::new(5);
//     println!("b = {b}");
// }

// // 使用cons list创建递归
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil))); // error[E0072]: recursive type `List` has infinite size
// }

// 使用cons list 和 Box<T> 创建递归
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
