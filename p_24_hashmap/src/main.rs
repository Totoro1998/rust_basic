use std::collections::HashMap;

// fn main() {
//     // 新建一个哈希 map
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     println!("{scores:#?}");

//     // 访问hashmap中的值
//     // let team_name = String::from("Blue");
//     // let score = scores.get(&team_name).copied().unwrap_or(0);

//     let team_name = String::from("Red");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
//     println!("{score}");
// }

// // 遍历HashMap
// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     // 元祖？
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

// // 哈希 map 和所有权
// fn main() {
//     use std::collections::HashMap;

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     println!("{field_name},{field_value}");
//     // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
//     map.insert(field_name, field_value);
//     // 这里 field_name 和 field_value 不再有效，
//     // 尝试使用它们看看会出现什么编译错误！
//     // println!("{field_name},{field_value}");
// }

// 当我们想要改变哈希 map 中的数据时，必须决定如何处理一个键已经有值了的情况。
// 可以选择完全无视旧值并用新值代替旧值。可以选择保留旧值而忽略新值，并只在键 没有 对应值时增加新值。或者可以结合新旧两值。
fn main() {
    let mut scores = HashMap::new();

    // // 覆盖一个值
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // // 只在键没有对应值时插入键值对
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // 根据旧值更新一个值,另一个常见的哈希 map 的应用场景是找到一个键对应的值并根据旧的值更新它。
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = scores.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{scores:?}");
}
