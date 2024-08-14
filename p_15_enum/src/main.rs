// fn main() {
//     #[derive(Debug)]
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
//     #[derive(Debug)]
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//     println!("home_kind---{:#?}", home.kind)
// }

// // 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。
// // 也就是说，IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用。
// // 作为定义枚举的结果，这些构造函数会自动被定义。
// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("home---{:?}", home); // home---V4("127.0.0.1")
//     println!("loopback---{:?}", loopback); // loopback---V6("::1")
// }

// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("home---{:?}", home); // home---V4(127, 0, 0, 1)
//     println!("loopback---{:?}", loopback); // loopback---V6("::1")
// }

fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("&self---{:?}", self) //&self---Write("hello")
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
