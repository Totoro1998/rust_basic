// #[inline]
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     let result = add(1, 2);
//     println!("Result: {}", result);
// }

#[inline(always)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let result = multiply(3, 4);
    println!("Result: {}", result);
}
