// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
// // 函数 area 本应该计算一个长方形的面积，不过函数却有两个参数。
// // 这两个参数是相关联的，不过程序本身却没有表现出这一点。
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // 使用元组重构
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// // 在计算面积时将宽和高弄混倒无关紧要，不过当在屏幕上绘制长方形时就有问题了！我们必须牢记 width 的元组索引是 0，height 的元组索引是 1。
// // 如果其他人要使用这些代码，他们必须要搞清楚这一点，并也要牢记于心。
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // 直接使用println!打印结构体是错误的。
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// // 可以加上:#?指示符告诉println! 我们想要使用叫做 Debug 的输出格式。然后在结构体定义之前加上外部属性 #[derive(Debug)]。
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("rect1 is {rect1:#?}");
// }

// // 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。
// // dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//     dbg!(&rect1);
// }
