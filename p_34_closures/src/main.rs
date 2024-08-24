// // 初识闭包
// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

// fn main() {
//     fn add_one_v1(x: u32) -> u32 {
//         x + 1
//     }
//     let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     let add_one_v3 = |x| x + 1;
//     let add_one_v4 = |x| x + 1;
//     add_one_v3(3); // 调用闭包是 add_one_v3 和 add_one_v4 能够编译的必要条件
//     add_one_v4(4); // 调用闭包是 add_one_v3 和 add_one_v4 能够编译的必要条件
// }

// // 闭包类型推断，当类型推断完成后，这个类型会被锁定，不能更改
// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));
//     // let n = example_closure(5);
// }

// // 捕获其环境中的值：不可变引用
// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let only_borrows = || println!("From closure: {list:?}");

//     println!("Before calling closure: {list:?}");
//     only_borrows();
//     println!("After calling closure: {list:?}");
// }

// // 捕获其环境中的值：可变引用
// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let mut borrows_mutably = || list.push(7);
//     println!("Before calling closure: {list:?}"); // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
//     borrows_mutably();
//     println!("After calling closure: {list:?}");
// }

// // 捕获其环境中的值：所有权
// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(move || println!("From thread: {list:?}"))
//         .join()
//         .unwrap();
//     // println!("After calling closure: {list:?}"); // error[E0382]: borrow of moved value: `list`
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// // 将被捕获的值移出闭包和Fntrait ：FnMut
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{list:#?}");
// }

// // 将被捕获的值移出闭包和Fntrait ：FnOnce
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("closure called");

//     list.sort_by_key(|r| {
//         sort_operations.push(value); // cannot move out of `value`, a captured variable in an `FnMut` closure
//         r.width
//     });
//     println!("{list:#?}");
// }

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
