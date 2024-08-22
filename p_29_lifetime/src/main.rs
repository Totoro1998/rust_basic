// //  悬垂引用: `x` does not live long enough
// fn main() {
//     let r; // r的生命周期开始
//     {
//         let x = 5; // x的生命周期开始
//         r = &x;
//     } // x的生命周期结束
//     println!("r: {r}");
//     // r的生命周期开始
// }

// // 没有产生悬垂引用且可以正确编译
// fn main() {
//     let x = 5; // x的生命周期开始
//     let r = &x; // r的生命周期开始
//     println!("r: {r}");
//     // r的生命周期结束
//     // x的生命周期结束
// }

// fn main() {
//     let string1 = String::from("xyz");
//     let result;
//     {
//         let string2 = "abcd";
//         result = longest(string1.as_str(), string2);
//     } // string2已经无效了
//     println!("The longest string is {result}"); // string2已经无效了
// }
// // 函数中的泛型生命周期：missing lifetime specifier
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// // 函数签名表达如下限制：也就是这两个参数和返回的引用存活的一样久。
// // 泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let string1 = String::from("xyz");
//     let string2 = "abcd";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result},string1---{string1},string2---{string2}");
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str()); //而 result 则引用了一些直到内部作用域结束都是有效的值
//         println!("The longest string is {result}");
//     } // string2 则在内部作用域中是有效的
//       // string1 直到外部作用域结束都是有效的
// }

// // error[E0597]: `string2` does not live long enough。
// // 因为result的生命周期和string2一样，取小。
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str()); //而 result 则引用了一些直到内部作用域结束都是有效的值
//     } // string2 则在内部作用域中是有效的
//       // string1 直到外部作用域结束都是有效的
//     println!("The longest string is {result}");
// }

// // 如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// fn main() {
//     let string1 = String::from("xyz");
//     let string2 = "abcd";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result},string1---{string1},string2---{string2}");
// }

// // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
// // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值。然而它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。
// // !注意这里说的是返回引用的情况，不考虑所有权。
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// // 可以定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解。
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     println!("i.part---{}", i.part)
// }

// // 方法定义中的生命周期
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     // 第一条生命周期省略规则规则
//     fn level(&self) -> i32 {
//         3
//     }
//     // 适用于第三条生命周期省略规则
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {announcement}");
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     let announcement = "abc";
//     println!("i.part---{}", i.part);
//     println!("level---{}", i.level());
//     println!(
//         "announce_and_return_part---{}",
//         i.announce_and_return_part(announcement)
//     )
// }

// // 静态生命周期其生命周期能够存活于整个程序期间。
// let s: &'static str = "I have a static lifetime.";

// 结合泛型类型参数、trait bounds 和生命周期
use std::fmt::Display;

// 因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = "Hello, Rust!";
    let string2 = "Hi!";

    let announcement = "Comparing two strings"; // announcement是字符串slice类型，也实现了Display

    let result = longest_with_an_announcement(string1, string2, announcement);

    println!("The longest string is: {}", result);
}
