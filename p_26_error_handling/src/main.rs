// fn main() {
//     panic!("crash and burn");
// }

// 可以使用 RUST_BACKTRACE=1 cargo run 来得到一个 backtrace。backtrace 是一个执行到目前位置所有被调用的函数的列表。
// Rust 的 backtrace 跟其他语言中的一样：阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。
// 这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
