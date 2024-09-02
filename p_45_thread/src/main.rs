use std::thread;
use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1)); // thread::sleep 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }

//     // handle.join().unwrap();
// }

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         // closure may outlive the current function, but it borrows `v`, which is owned by the current function
//         println!("Here's a vector: {v:?}");
//     });

//     handle.join().unwrap();
// }

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
