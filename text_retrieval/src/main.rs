// in main.rs
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("错误信息: {err}");
        process::exit(1);
    });

    println!("搜索参数 {}", config.query);
    println!("文件名 {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("程序错误: {e}");
        process::exit(1);
    }
}
