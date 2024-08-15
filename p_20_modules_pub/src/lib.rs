// // 在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的。
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// // 使模块公有并不使其内容也是公有的。模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码
// mod front_of_house {
//     pub mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// // super 开始的相对路径
// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// // 公有的结构体
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         // 因为 back_of_house::Breakfast 具有私有字段，所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例 (这里我们命名为 summer)。
//         // 如果 Breakfast 没有这样的函数，我们将无法在 eat_at_restaurant 中创建 Breakfast 实例，因为我们不能在 eat_at_restaurant 中设置私有字段 seasonal_fruit 的值。
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // 在夏天订购一个黑麦土司作为早餐
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 改变主意更换想要面包的类型
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // 如果取消下一行的注释代码不能编译；
//     // 不允许查看或修改早餐附带的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");
// }

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
