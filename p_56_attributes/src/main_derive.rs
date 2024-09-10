// !通过#[derive(...)]，Rust编译器会根据类型的定义自动实现指定的trait
// #[derive(Debug, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let p1 = Point { x: 10, y: 20 };

//     // 使用Debug进行打印
//     println!("{:?}", p1);

//     // 使用Clone进行拷贝
//     let p2 = p1.clone();
//     println!("{:?}", p2);
// }

#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 10, y: 20 };

    // 比较两个结构体是否相等
    if p1 == p2 {
        println!("p1 and p2 are equal.");
    } else {
        println!("p1 and p2 are not equal.");
    }
}
