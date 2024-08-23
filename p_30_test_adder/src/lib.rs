// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// // 自定义测试
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail"); // thread 'tests::another' panicked at src/lib.rs:28:9:
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// // 使用 assert!
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }

// // 自定义失败信息
// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`",
//             result
//         );
//     }
// }

pub struct Guess {
    value: i32,
}

// // 使用#[should_panic]
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// // 使用 should_panic 属性增加的一个可选的 expected 参数
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be greater than or equal to 1, got {value}.");
//         } else if value > 100 {
//             panic!("Guess value must be less than or equal to 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//         // Guess::new(-1);
//     }
// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
// // 将Result<T, E>用于测试
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// // 显示函数输出
// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {a}");
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// 运行单个测试：cargo test one_hundred
// 过滤运行多个测试：cargo test add cargo test tests
// 运行被忽略的测试：cargo test -- --ignored
// 不管是否忽略都要运行全部测试：可以运行 cargo test -- --include-ignored。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    #[test]
    #[ignore] //除非特别指定否则忽略某些测试
    fn expensive_test() {
        assert_eq!(102, add_two(100));
    }
}
