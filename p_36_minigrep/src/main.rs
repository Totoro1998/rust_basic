use std::env;
use std::process;

use p_36_minigrep::Config;

//大小写敏感： IGNORE_CASE=1 cargo run to poem.txt
//大小写不敏感： cargo run to poem.txt
//将错误打印到标准错误： cargo run > output.txt
//成功运行所产生的输出重定向到文件中： cargo run -- to poem.txt > output.txt
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = p_36_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
