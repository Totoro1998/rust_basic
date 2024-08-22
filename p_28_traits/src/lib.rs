use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现
    fn default_summarize(&self) -> String {
        String::from("(default_summarize...)")
    }
    // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。
    fn summarize_other(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 需要注意的限制是，只有在 trait 或类型至少有一个属于当前 crate 时，我们才能对类型实现该 trait。
// 不能为外部类型实现外部 trait。例如，不能在 p-28_traits crate 中为 Vec<T> 实现 Display trait。
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义 Tweet 的显示格式
        writeln!(
            f,
            "@{}: {}\n(reply: {}, retweet: {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}
