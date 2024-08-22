use std::fmt::Display;

use p_28_traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // println!("1 new tweet: {}", tweet.summarize());
    // println!("1 new article: {}", article.default_summarize());
    // println!("1 new tweet: {}", tweet.summarize_other());
    // let test = String::from("test");
    // notify(&test); // the trait bound `String: Summary` is not satisfied
    // notify(&tweet);
    // notify(&tweet, &article);
    // notify(&tweet, &tweet);
    // notify(&article);
    // notify(&tweet);
    notify(&tweet, &article);
    let returned = returns_summarizable();
    println!("returned summarizable: {}", returned.summarize());
}

// // trait 作为参数
// // item参数是实现了 Summary trait 的某种类型，如果类型没有实现Summary trait就不能调用该方法
// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 上述代码其实是 Trait Bound 语法的语法糖
// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 这适用于 item1 和 item2 允许是不同类型的情况
// fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("item1 Breaking news! {}", item1.summarize());
//     println!("item2 Breaking news! {}", item2.summarize());
// }

// // 泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
// fn notify<T: Summary>(item1: &T, item2: &T) {
//     // expected `&Tweet`, found `&NewsArticle`
//     println!("item1 Breaking news! {}", item1.summarize());
//     println!("item2 Breaking news! {}", item2.summarize());
// }

// // 通过 + 指定多个 trait bound
// fn notify(item: &(impl Summary + Display)) {
//     println!("item Breaking news! {}", item);
// }
// // 通过 + 指定多个 trait bound
// fn notify<T: Summary + Display>(item: &T) {
//     println!("item Breaking news! {}", item);
// }

// 通过 where 简化 trait bound
fn notify<T, U>(item1: &T, item2: &U)
where
    T: Display + Summary,
    U: Summary,
{
    println!("item1 Breaking news! {}", item1);
    println!("item2 Breaking news! {}", item2.summarize());
}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// // 不过这只适用于返回单一类型的情况。例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就行不通
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// // 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// // 类型 Pair<T> 总是实现了 new 方法并返回一个 Pair<T> 的实例
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// // 只有那些为 T 类型实现了 PartialOrd trait（来允许比较） 和 Display trait（来启用打印）的 Pair<T> 才会实现 cmp_display 方法
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，它们被广泛的用于 Rust 标准库中。
trait MySummary {
    fn summarize(&self) -> String;
}

trait MyTest {
    fn test(&self) -> String;
}

impl<T: MySummary> MyTest for T {
    fn test(&self) -> String {
        format!("Testing: {}", self.summarize())
    }
}

struct Article {
    title: String,
    content: String,
}

impl MySummary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// fn main() {
//     let article = Article {
//         title: String::from("Rust Programming"),
//         content: String::from("Rust is fast and memory efficient."),
//     };
//     // 因为 Article 实现了 MySummary
//     // 所以它自动实现了 MyTest
//     println!("{}", article.test());
// }
