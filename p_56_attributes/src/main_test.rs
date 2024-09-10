// !在Rust中，#[test] 属性用于标记测试函数，它通常与Rust的内置测试框架结合使用。测试函数可以验证代码的正确性，通过cargo test命令运行这些测试
#[cfg(test)] // 条件编译，确保测试代码只在测试时编译
mod tests {
    // 标记这是一个测试函数
    #[test]
    fn it_works() {
        // 断言2 + 2是否等于4
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        // 使用assert宏验证条件
        let value = 10;
        assert!(value > 5);
    }
}
