use std::cell::RefCell;

// 假设我们有一个不可变的结构体Person，其中包含一个可变的年龄字段。
// 如果不使用RefCell<T>，我们需要将整个结构体实例标记为可变（mut），才能修改年龄。但是，使用RefCell<T>，我们可以在结构体不可变的情况下，修改内部的年龄字段。
struct Person {
    name: String,
    age: RefCell<u32>,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: RefCell::new(age),
        }
    }

    // 增加年龄的方法
    fn have_birthday(&self) {
        // 这里获取age的可变借用并增加值
        *self.age.borrow_mut() += 1;
    }

    // 获取当前年龄的方法
    fn age(&self) -> u32 {
        // 这里获取age的不可变借用并返回值
        // self.age.borrow() // expected `u32`, found `Ref<'_, u32>`
        *self.age.borrow()
    }
}

fn main() {
    let person = Person::new("Alice", 30);

    println!(
        "Before birthday: {} is {} years old.",
        person.name,
        person.age()
    );

    person.have_birthday();

    println!(
        "After birthday: {} is now {} years old.",
        person.name,
        person.age()
    );
}
