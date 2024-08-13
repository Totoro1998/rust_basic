// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 定义常量
//     println!("The THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS)
// }
// // The THREE_HOURS_IN_SECONDS is 10800

// fn main() {
//     // let x = 5;
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x = 5;
    let x = x + 1;
    // {}代表单独的作用域
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
// The value of x in the inner scope is: 12
// The value of x is: 6

// fn main() {
//     let spaces = "123";
//     let spaces = spaces.len();
//     println!("The spaces is {}", spaces)
// }
// // The spaces is 3
