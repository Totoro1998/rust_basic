// fn main() {
//     // 创建vector
//     // let v: Vec<i32> = Vec::new();
//     let mut v = vec![1, 2, 3];

//     // 更新vector
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//     println!("{v:#?}");

//     // 读取vector
//     let third: &i32 = &v[2];
//     println!("The third element is {third}");

//     let third: Option<&i32> = v.get(2); // 当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>。
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
//     v.push(6)

//     println!("The first element is: {first}");
// }

// // 遍历元素
// fn main() {
//     // let v: Vec<i32> = vec![100, 32, 57];
//     // for i in &v {
//     //     println!("{i}");
//     // }

//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
//     }
//     println!("{v:#?}");
// }

// 使用枚举来储存多种类型
fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{row:#?}");
}
