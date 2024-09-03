// // 函数指针
// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("The answer is: {answer}");
// }

// 既可以使用内联定义的闭包又可以使用命名函数
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

// // 返回闭包
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }
// // 返回闭包
// fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
//     Box::new(|x| x + 1)
// }
