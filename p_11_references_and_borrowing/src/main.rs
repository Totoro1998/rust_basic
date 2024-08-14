// // 初识引用
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1); // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。

//     println!("The length of '{s1}' is {len}."); // 因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
// }

// // 以一个对象的引用作为参数而不是获取值的所有权
// // 函数签名使用 & 来表明参数 s 的类型是一个引用
// fn calculate_length(s: &String) -> usize {
//     // s 是 String 的引用
//     s.len()
// } //这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

// // 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// // 可变引用
// fn main() {
//     let mut s = String::from("hello"); // 将 s 改为 mut
//     change(&mut s); // 创建一个可变引用 &mut s
// }

// // 新函数签名以接受一个可变引用 some_string: &mut String
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// // 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     // let r2 = &s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//         println!("inner r1---{r1}")
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     let r2 = &mut s;
//     println!("outer r2---{r2}")
// }

// // 我们也不能在拥有不可变引用的同时拥有可变引用。
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     let r3 = &mut s; // 大问题

//     println!("{}, {}, and {}", r1, r2, r3);
// }

// // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用

//     let r3 = &mut s; // 没问题
//     println!("{}", r3);
// }

// // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
// // 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
// // 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
// fn main() {
//     let reference_to_nothing = dangle();
// }

// // dangle 返回一个字符串的引用
// fn dangle() -> &String {
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 返回字符串 s 的引用
//        // s // 解决方法是直接返回 String
// } // 这里 s 离开作用域并被丢弃。其内存被释放。危险！

// // Slice语法
// fn main() {
//     let s = String::from("hello world");
//     let len = s.len();
//     // let hello = &s[0..5];
//     let hello = &s[..5];
//     // let world = &s[6..11];
//     // let world = &s[6..];
//     let world = &s[6..len];
//     let slice = &s[..];

//     println!("hello---{hello},world---{world},slice---{slice}")
// }

// fn main() {
//     let s = String::from("hello world");
//     let word = first_word(&s);
//     // 错误！当拥有某值的不可变引用时，就不能再获取一个可变引用。
//     // 因为 clear 需要清空 String，它尝试获取一个可变引用。\
//     // 在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。
//     // s.clear();
//     println!("the first word is: {}", word);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` 适用于 `String`（的 slice），部分或全部
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` 也适用于 `String` 的引用，
//     // 这等价于整个 `String` 的 slice
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` 适用于字符串字面值，部分或全部
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // 因为字符串字面值已经 **是** 字符串 slice 了，
//     // 这也是适用的，无需 slice 语法！
//     let word = first_word(my_string_literal);
// }

// // 修改first_word参数的签名：s: &String改为s: &str
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// 其他类型的slice
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
