use std::slice;

// // 解引用裸指针
// fn main() {
//     let mut num = 5;

//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }
// }

// // 创建不安全代码的安全抽象
// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];

//     // let (a, b) = r.split_at_mut(3);
//     let (a, b) = split_at_mut(&mut v, 3);

//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }
// // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// //     let len = values.len();

// //     assert!(mid <= len);

// //     (&mut values[..mid], &mut values[mid..]) // cannot borrow `*values` as mutable more than once at a time
// // }
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// // 使用 extern 函数调用外部代码
// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }

// // 访问或修改可变静态变量
// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// fn main() {
//     add_to_count(3);

//     unsafe {
//         println!("COUNTER: {COUNTER}");
//     }
// }

// 实现不安全 trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
