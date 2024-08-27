struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// // 当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃。
// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     println!("CustomSmartPointers created.");
// }

// // CustomSmartPointers created.
// // Dropping CustomSmartPointer with data `other stuff`!
// // Dropping CustomSmartPointer with data `my stuff`!

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    drop(c); // 通过std::mem::drop提早丢弃值
    println!("CustomSmartPointer dropped before the end of main.");
}

// CustomSmartPointer created.
// Dropping CustomSmartPointer with data `my stuff`!
// CustomSmartPointer dropped before the end of main.
// Dropping CustomSmartPointer with data `other stuff`!
