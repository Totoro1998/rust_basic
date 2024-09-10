// !在Rust中，#[cfg(...)] 是一个用于条件编译的属性，它允许根据特定条件编译和运行代码。#[cfg(...)] 通常用来检查操作系统、编译目标、特定的编译特性或自定义的编译标志，进而决定是否包含某段代码。

// // 操作系统检测
// fn main() {
//     platform_specific_function();
// }

// #[cfg(target_os = "windows")]
// fn platform_specific_function() {
//     println!("Running on Windows!");
// }

// #[cfg(target_os = "linux")]
// fn platform_specific_function() {
//     println!("Running on Linux!");
// }

// #[cfg(target_os = "macos")]
// fn platform_specific_function() {
//     println!("Running on macOS!");
// }

// // 可以使用#[cfg(feature = "feature_name")]来检测是否启用了特定的编译特性
// // Cargo.toml
// // [features]
// // default = []
// // advanced_math = []
// // 通过 cargo run --features "advanced_math"，可以启用 advanced_math 特性
// // 如果特性启用了，advanced_math函数会被编译并运行；否则，它会被忽略
// fn main() {
//     basic_math();

//     #[cfg(feature = "advanced_math")]
//     advanced_math();
// }

// fn basic_math() {
//     println!("Performing basic math!");
// }

// #[cfg(feature = "advanced_math")]
// fn advanced_math() {
//     println!("Performing advanced math!");
// }

// 可以使用all、any、not可以组合多个条件
#[cfg(all(target_os = "linux", feature = "advanced_math"))]
fn linux_advanced_math() {
    println!("Advanced math on Linux!");
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn cross_platform_function() {
    println!("This runs on Windows or macOS.");
}

fn main() {
    #[cfg(all(target_os = "linux", feature = "advanced_math"))]
    linux_advanced_math();

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    cross_platform_function();
}
