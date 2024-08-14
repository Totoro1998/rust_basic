// fn main() {
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter,
//     }

//     fn value_in_cents(coin: Coin) -> u8 {
//         // coin代表一个表达式的值
//         match coin {
//             // 四个分支
//             Coin::Penny => {
//                 println!("Lucky penny!");
//                 1
//             } // 如果用了大括号，可以不使用逗号
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter => 25,
//         }
//     }

//     println!("Coin::Penny.value---{}", value_in_cents(Coin::Penny));
//     println!("Coin::Nickel.value---{}", value_in_cents(Coin::Nickel));
//     println!("Coin::Dime.value---{}", value_in_cents(Coin::Dime));
//     println!("Coin::Quarter.value---{}", value_in_cents(Coin::Quarter));
// }

// fn main() {
//     #[derive(Debug)] // 这样可以立刻看到州的名称
//     enum UsState {
//         Alabama,
//         Alaska,
//         // --snip--
//     }

//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }

//     // 想象一下我们的一个朋友尝试收集所有 50 个州的 25 美分硬币。
//     // 在根据硬币类型分类零钱的同时，也可以报告出每个 25 美分硬币所对应的州名称，这样如果我们的朋友没有的话，他可以将其加入收藏。
//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             // 像这样就可以获取 Coin 枚举的 Quarter 成员中内部的州的值。
//             Coin::Quarter(state) => {
//                 println!("State quarter from {state:?}!"); // State quarter from Alaska!
//                 25
//             }
//         }
//     }
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// // 匹配 Option<T>
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("six---{six:?}"); // six---Some(6)
//     println!("none---{none:?}"); // none---None
// }

// // 匹配是穷尽的
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             Some(i) => Some(i + 1),
//         }
//     }
// }

// // 通配符
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {
//         println!("num_spaces---{num_spaces}")
//     }
// }

// // 占位符
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {
//         println!("nothing to do")
//     }
// }

// fn main() {
//     // let config_max = Some(3u8);
//     // match config_max {
//     //     Some(max) => println!("The maximum is configured to be {max}"),
//     //     _ => (),
//     // }
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {max}"); // The maximum is configured to be 3
//     }
// }

// fn main() {
//     let num = 5;
//     if let 5 = num {
//         println!("The num is  {num}");
//     }
// }

// fn main() {
//     #[derive(Debug)]
//     enum UsState {
//         Alabama,
//         Alaska,
//     }
//     #[derive(Debug)]
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }

//     let mut count = 1;
//     let coin = Coin::Penny;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {state:?}!");
//     } else {
//         count += 1;
//     }
//     println!("count---{count}")
// }
