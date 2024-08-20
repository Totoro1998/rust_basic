// // 使用 to_string 方法从string字面量创建 String
// fn main() {
//     let data = "initial contents";

//     let s = data.to_string();

//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
//     println!("{s}")
// }

// // UTF-8 编码
// fn main() {
//     let hello = String::from("السلام عليكم");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שלום");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     // let hello = String::from("Hola");
//     println!("{hello}")
// }

// // 使用 push_str 和 push 附加 String
// fn main() {
//     // let mut s = String::from("foo");
//     // s.push_str("bar");

//     // println!("{s}")

//     // push_str 方法采用string slice，因为我们并不需要获取参数的所有权。
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {s2}");

//     // push 方法被定义为获取一个单独的 character 作为参数，并附加到 String 中：

//     let mut s = String::from("lo");
//     s.push('l');
// }

// // 使用+运算符
// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

//     println!("{s3}");
//     // println!("{s1}");
// }

// // 使用format!宏
// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{s1}-{s2}-{s3}");
//     println!("{s1},{s2},{s3},{s}");
// }

// fn main() {
//     // let s1 = String::from("hello");
//     // let h = s1[0]; //Rust 的strings不支持索引

//     let hello = String::from("Hola");
//     let len: usize = hello.len();
//     println!("hello len---{len}"); // 4
//     let hello = String::from("Здравствуйте");
//     let len: usize = hello.len();
//     // 这是使用 UTF-8 编码 “Здравствуйте” 所需要的字节数，因为在这个string中每个 Unicode 标量值需要两个字节存储。
//     println!("hello len---{len}"); // 24
//     let hello = "Здравствуйте";
//     let s = &hello[0..4];
//     println!("s---{s}"); // s---Зд
// }

// 遍历 Strings
fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }
    // З
    // д

    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 208
    // 151
    // 208
    // 180
}
