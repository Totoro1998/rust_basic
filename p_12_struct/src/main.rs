struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// // 定义结构体和实例化
// fn main() {
//     let user = User {
//         active: true,
//         username: String::from("fk"),
//         email: String::from("fk@example.com"),
//         sign_in_count: 1,
//     };
//     println!(
//         "user.active---{},user.username---{},user.email---{},user.sign_in_count---{}",
//         user.active, user.username, user.email, user.sign_in_count
//     );
// }

// // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
// fn main() {
//     let mut user = User {
//         active: true,
//         username: String::from("fk"),
//         email: String::from("fk@example.com"),
//         sign_in_count: 1,
//     };

//     user.email = String::from("fkchange@example.com");

//     println!(
//         "user.active---{},user.username---{},user.email---{},user.sign_in_count---{}",
//         user.active, user.username, user.email, user.sign_in_count
//     );
// }

// // 使用字段初始化简写语法
// fn main() {
//     // let user = build_user("fk@example.com", "fk");
//     let email = String::from("fkchange@example.com");
//     let username = String::from("fk");
//     let user = build_user(email, username);
//     println!(
//         "user.active---{},user.username---{},user.email---{},user.sign_in_count---{}",
//         user.active, user.username, user.email, user.sign_in_count
//     );
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("fk"),
//         email: String::from("fk@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     // 在这个例子中，总体上说我们在创建 user2 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。
//     println!(
//         "user1.active---{},user1.username---{},user1.email---{},user1.sign_in_count---{}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
//     println!(
//         "user2.active---{},user2.username---{},user2.email---{},user2.sign_in_count---{}",
//         user2.active, user2.username, user2.email, user2.sign_in_count
//     );
// }

// // 使用结构体更新语法从其他实例创建实例
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("fk"),
//         email: String::from("fk@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     // 在这个例子中，总体上说我们在创建 user2 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。
//     println!(
//         "user1.active---{},user1.username---{},user1.email---{},user1.sign_in_count---{}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
//     println!(
//         "user2.active---{},user2.username---{},user2.email---{},user2.sign_in_count---{}",
//         user2.active, user2.username, user2.email, user2.sign_in_count
//     );
// }

// // 使用结构体更新语法从其他实例创建实例
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("fk"),
//         email: String::from("fk@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         username: String::from("anothername@example.com"),
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     // 如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。
//     println!(
//         "user1.active---{},user1.username---{},user1.email---{},user1.sign_in_count---{}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
//     println!(
//         "user2.active---{},user2.username---{},user2.email---{},user2.sign_in_count---{}",
//         user2.active, user2.username, user2.email, user2.sign_in_count
//     );
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// // 元祖结构体
// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("black[0]---{},origin[0]---{}", black.0, origin.0)
// }

// struct AlwaysEqual;
// // 类单元结构体
// fn main() {
//     let subject = AlwaysEqual;
// }

// 获取self的所有权
fn main() {
    pub struct Queue {
        older: Vec<char>,   // 较旧的元素，最早进来的在后面
        younger: Vec<char>, // 较新的元素，最后进来的在后面
    }
    impl Queue {
        /// 把字符推入队列的最后
        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }
        /// 从队列的前面弹出一个字符。如果确实有要弹出的字符，
        /// 就返回`Some(c)`；如果队列为空，则返回`None`
        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                // 将younger中的元素移到older中，并按照所承诺的顺序排列它们
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            // 现在older能保证有值了。Vec的pop方法已经
            // 返回一个Option，所以可以放心使用了
            self.older.pop()
        }
    }
    impl Queue {
        pub fn split(self) -> (Vec<char>, Vec<char>) {
            (self.older, self.younger)
        }
    }

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split();
    // q现在是未初始化状态
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
