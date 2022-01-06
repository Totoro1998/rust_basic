fn main() {
    print_labeled_measurement(5, 'h');
    let x = five();
    let y = plus_one(6);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
fn five() -> i32 {
    5 //不能加;
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
