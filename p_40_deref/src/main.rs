// // 使用解引用运算符来跟踪所引用的值
// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     // assert_eq!(5, y); // 不允许比较数字的引用与数字，因为它们是不同的类型。必须使用解引用运算符追踪引用所指向的值。
//     assert_eq!(5, *y);
// }

// // 像引用一样使用 Box<T>
// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// // 自定义智能指针
// use std::ops::Deref;
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// // 实现 Deref
// impl<T> Deref for MyBox<T> {
//     type Target = T; // type Target = T; 语法定义了用于此 trait 的关联类型

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

use std::ops::{Deref, DerefMut};

// 定义一个包含泛型类型 T 的结构体
struct MyBox<T>(T);

// 为 MyBox 实现 Deref trait，将 Target 设为泛型 T
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 为 MyBox 实现 DerefMut trait
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let x = MyBox(5); // `MyBox<i32>`
    let y: &i32 = &x; // 从 `&MyBox<i32>` 强制转换为 `&i32`
    println!("y: {}", y);

    let mut z = MyBox(10); // `MyBox<i32>`
    let w: &mut i32 = &mut z; // 从 `&mut MyBox<i32>` 强制转换为 `&mut i32`
    println!("w: {}", w);

    let v: &i32 = &mut z; // 从 `&mut MyBox<i32>` 强制转换为 `&i32`
    println!("v: {}", v);
}
