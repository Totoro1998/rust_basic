// 创建一个某项知道其子项和父项的树形结构
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }
// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         children: RefCell::new(vec![]),
//     });

//     let branch = Rc::new(Node {
//         value: 5,
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
// }

// 增加从子到父的引用
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });

//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// }

// 可视化 strong_count 和 weak_count 的改变
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
