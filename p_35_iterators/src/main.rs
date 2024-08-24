// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter(); // 创建一个迭代器，迭代器是惰性的。

//     // 消费迭代器
//     for val in v1_iter {
//         println!("Got: {val}");
//     }
// }

// fn main() {
//     let v1 = vec![String::from("1"), String::from("2"), String::from("3")];

//     let v1_iter = v1.iter(); // 创建一个迭代器，迭代器是惰性的。从 next 调用中获取的值是对 vector 中值的不可变引用。iter 方法生成一个不可变引用的迭代器。

//     // 消费迭代器
//     for val in v1_iter {
//         // 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
//         println!("Got: {val}");
//     }
//     // println!("v1_iter---{v1_iter:?}"); //for 循环会获取 v1_iter 的所有权: error[E0382]: borrow of moved value: `v1_iter`
// }

// fn main() {
//     let v1 = vec![1, 2, 3];

//     // 注意我们需要将 v1_iter 声明为可变的，在迭代器上调用 next 方法会改变迭代器内部的状态，该状态用于跟踪迭代器在序列中的位置。
//     let mut v1_iter = v1.iter(); // 创建一个迭代器，迭代器是惰性的。

//     // next 方法，该方法每次返回迭代器中的一个项，封装在 Some 中，并且当迭代完成时，返回 None。
//     v1_iter.next();
//     println!("v1_iter---{v1_iter:?}");
//     v1_iter.next();
//     println!("v1_iter---{v1_iter:?}");
//     v1_iter.next();
//     println!("v1_iter---{v1_iter:?}");
// }

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     // let total: i32 = v1_iter.clone().sum(); // 克隆迭代器

//     println!("{total}");
//     // println!("{v1_iter:?}"); //调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。 borrow of moved value: `v1_iter`

//     // 重新创建一个迭代器
//     let v1_iter_again = v1.iter();
//     println!("{:?}", v1_iter_again);
// }

// 产生其他迭代器的方法：迭代器适配器
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{v2:?}");
}
