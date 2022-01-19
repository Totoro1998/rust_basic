fn main() {
    string_test();
    string_clone_test();
    reference_test();
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    change_reference_test(&mut s);
    println!("{}", s);
}
fn string_test() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}
fn string_clone_test() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
fn reference_test() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change_reference_test(some_string: &mut String) {
    some_string.push_str(", world");
}
// Slice类型
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
