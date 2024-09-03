// 定义一个带有 draw 方法的 trait Draw
pub trait Draw {
    fn draw(&self);
}

// 定义了一个存放了名叫 components 的 vector 的结构体 Screen
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //这个 vector 的类型是 Box<dyn Draw>，此为一个 trait 对象：它是 Box 中任何实现了 Draw trait 的类型的替身。
}

impl Screen {
    // 该方法会对其 components 上的每一个组件调用 draw 方法
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button-width-{},Button-height-{},Button-label-{}",
            self.width, self.height, self.label
        )
    }
}
