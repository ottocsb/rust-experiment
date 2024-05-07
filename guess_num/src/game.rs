use colored::*;
use rand::prelude::*;
use std::io;

// 创建一个枚举 游戏状态
enum GameStatus {
    Run(String),
    Over(String),
}

// 创建一个结构体 游戏体
pub struct Game {
    guess_num: i32,
    rand_num: i32,
    count: i32,
}

// 实现结构体方法
impl Game {
    // 初始化实例方法
    pub fn new() -> Game {
        Game {
            guess_num: 0,
            rand_num: thread_rng().gen_range(0..101),
            count: 0,
        }
    }

    // 执行方法
    pub fn run(&mut self) {
        println!(
            "{}",
            "猜数字游戏开始! 系统会生成1-100之间的一个随机数，你有五次输入机会!".yellow()
        );

        loop {
            self.count += 1;
            if self.count > 5 {
                println!(
                    "你已经猜了5次了，系统生成的随机数是：{}，游戏结束!",
                    self.rand_num.to_string().red()
                );
                break;
            }
            println!("第{}次猜测", self.count.to_string().red());
            println!("请输入你猜的数字:");

            self.input_num();
            let status = self.once();
            match status {
                GameStatus::Run(word) => {
                    println!("{}", word.red());
                }
                GameStatus::Over(word) => {
                    println!("{}", word.green());
                    break;
                }
            }
        }
    }

    // 逻辑判断
    fn once(&mut self) -> GameStatus {
        if self.guess_num == self.rand_num {
            GameStatus::Over(String::from("你猜对了! "))
        } else {
            if self.guess_num > self.rand_num {
                GameStatus::Run(String::from("太大了!\n"))
            } else {
                GameStatus::Run(String::from("太小了!\n"))
            }
        }
    }

    // 输入方法
    fn input_num(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        self.guess_num = input.trim().parse().expect("Please type a number!");
    }
}
