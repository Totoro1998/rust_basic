use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); // Rc<List>

    println!("a initial rc count = {}", Rc::strong_count(&a)); // a initial rc count = 1
    println!("a next item = {:?}", a.tail()); // a next item = Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // Rc<List>

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // a rc count after b creation = 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // b initial rc count = 1
    println!("b next item = {:?}", b.tail()); // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // b rc count after changing a = 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // a rc count after changing a = 2

    // 如果取消注释这一行，打印 a 的下一个节点会导致无限循环，因为 a 的下一个节点是 b，而 b 的下一个节点又是 a，从而形成了一个环。
    // 这会导致栈溢出错误（stack overflow），因为循环会无限制地进行下去。
    // println!("a next item = {:?}", a.tail());
}
