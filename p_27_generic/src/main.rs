// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
// }

// // 实现泛型版本的largest函数
// // 不过还不能编译成功： binary operation `>` cannot be applied to type `&T`
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     // let wont_work = Point { x: 5, y: 4.0 };
//     // println!("{integer:#?}");
//     // println!("{float:#?}");
// }

// // 使用多个泛型类型参数
// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
//     println!("{both_integer:#?}");
//     println!("{both_float:#?}");
//     println!("{integer_and_float:#?}");
// }

// // 枚举定义中的泛型
// enum Option<T> {
//     Some(T),
//     None,
// }
// // 当成功打开文件的时候，T 对应的是 std::fs::File 类型；而当打开文件出现问题时，E 的值则是 std::io::Error 类型。
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// // 方法定义中的泛型
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同的名称
// impl<U> Point<U> {
//     fn get_x(&self) -> &U {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.get_x());
// }

// // 定义方法时也可以为泛型指定限制（constraint）。例如，可以选择为 Point<f32> 实例实现方法，而不是为泛型 Point 实例。
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let p = Point { x: 5.0, y: 10.0 };
//     // let p = Point { x: 5.0, y: 10 };
//     println!("p.distance = {}", p.distance_from_origin());
// }

// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// impl<G1, G2> Point<G1, G2> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<G1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 }; // 定义了一个有i32类型的x和f64类型的y的Point
    let p2 = Point { x: "Hello", y: 'c' }; // 定义了一个有字符串slice类型的x和char 类型的y的Point
                                           // 在 p1 上以 p2 作为参数调用 mixup 会返回一个 p3，它会有一个 i32 类型的 x，因为 x 来自 p1，并拥有一个 char 类型的 y，因为 y 来自 p2。
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
