// // 变量作用域
// fn main() {
//     {
//         // println!("before s---{s}") // s 在这里无效，它尚未声明
//         let s = "hello"; // 从此处起，s 是有效的
//         println!("inner s---{s}") // 使用 s
//     } // 此作用域已结束，s 不再有效
//       // println!("outer s---{s}")
// }

// // 使用 String
// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!"); // push_str() 在字符串后追加字面值
//     println!("{}", s); // 将打印 `hello, world!`
// }

// // 变量与数据交互的方式（一）：移动：Rust 永远也不会自动创建数据的 “深拷贝”。
// fn main() {
//     // let x = 5;
//     // let y = x;
//     // println!("x---{x},y---{y}") // x---5,y---5

//     let s1 = String::from("hello"); //String 由三部分组成，一个指向存放字符串内容内存的指针(ptr)，一个长度(len)，和一个容量(capacity)。
//     let s2 = s1;

//     // println!("{s1}, world!"); // 为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效
//     println!("{s2}, world!");
// }

// // 变量与数据交互的方式（二）：克隆
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// // 只在栈上的数据：拷贝
// // 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
// // Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
// fn main() {
//     let x = 5;
//     let y = x;
//     println!("x = {x}, y = {y}");
// }

// // 所有权与函数
// fn main() {
//     let s = String::from("hello"); // s 进入作用域
//     takes_ownership(s); // s 的值移动到函数里 ...
//                         // println!("s---{}", s); // ... 所以到这里不再有效

//     let x = 5; // x 进入作用域
//     makes_copy(x); // x 应该移动函数里
//                    // println!("x---{}", x); // 但 i32 是 Copy 的，所以在后面可继续使用 x。
// } // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，不会做特殊处理。

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放。

// fn makes_copy(some_integer: i32) {
//     // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会做特殊处理。

// // 返回值与作用域
// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值，转移给 s1
//     let s2 = String::from("hello"); // s2 进入作用域
//     let s3 = takes_and_gives_back(s2); // s2 被移动到takes_and_gives_back 中，被移动到takes_and_gives_back也将返回值移给 s3
//     println!("s3---{s3}");
//     // println!("s2---{s2}");
//     println!("s1---{s1}");
// } // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走到s3，所以什么也不会发生。s1 离开作用域并被丢弃

// fn gives_ownership() -> String {
//     // gives_ownership 会将返回值移动给调用它的函数
//     let some_string = String::from("yours"); // some_string 进入作用域。
//     some_string // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域
//     a_string // 返回 a_string 并移出给调用的函数
// }

// 如果我们想要函数使用一个值但不获取所有权该怎么办呢？
// 如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。
// 我们可以使用元组来返回多个值
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}