fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(5, 'h');
    expressions();
    let x = five();
    let y = plus_one(5);
    let z = plus_two(5);
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

// 定义函数
fn another_function() {
    println!("Another function.");
}

// 传递参数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

// 表达式
fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

// 函数返回值
fn five() -> i32 {
    5
}
// 表达式与语句的区别，表达式返回值，不能加分号。
fn plus_one(x: i32) -> i32 {
    x + 1
}
// 但如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }

fn plus_two(x: i32) -> i32 {
    return x + 2;
}
