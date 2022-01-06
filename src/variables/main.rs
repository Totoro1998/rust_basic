const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
    let x = 5;
    println!("The value of x is: {}", x); //5
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); //12
    }
    let mut y = 1;
    println!("The value of y is: {}", y);
    y = 2;
    println!("The value of y  is: {}", y);
}
