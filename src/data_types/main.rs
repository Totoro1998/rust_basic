fn main() {
    //整型
    let i = -10;
    //浮点型
    let y: f32 = 3.0;
    //布尔型，显式指定类型注解
    let f: bool = false;
    println!("The value of f is: {}", f);
    //字符类型
    let c = '中';
    println!("The value of c is: {}", c);
    //元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);
    //数组
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; //更简洁
    println!("The first value of months is: {}", months[0]);
    println!("The first value of arr is: {}", arr[0]);
}
