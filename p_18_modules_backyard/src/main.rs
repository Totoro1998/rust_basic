use crate::garden::vegetables::Asparagus;

pub mod garden; // 告诉编译器应该包含在src/garden.rs文件中发现的代码

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
