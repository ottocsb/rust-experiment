mod game;
mod sort;

use rand::prelude::*;
// use std::{io, thread};
use std::io;
// use lazy_static::lazy_static;
use game::Game;
// use sort::{bubble_sort, quick_sort};
// use std::sync::Mutex;

/// 创建随机数方法，接收一个可变引用参数，将参数变成随机数
/// 这里只是为了学习rust的可变引用参数 简版直接使用thread_rng().gen_range(1..101)即可
fn _create_rand_num(num: &mut i32) {
    *num = thread_rng().gen_range(1..101);
}

/// 效验方法 接收三个参数，一个是用户输入的数字，一个是随机数，还有一个是提示词
/// 提示词为可变引用参数，如果用户输入的数字大于随机数，提示词为大了，反之为小了
/// 如果相等则提示猜对了
fn _check_num(input: i32, num: i32, hint: &mut String) {
    if input == num {
        *hint = String::from("你猜对了!");
    } else {
        *hint = if input > num {
            String::from("太大了!")
        } else {
            String::from("太小了!")
        };
    }
}

fn _guess_num() {
    let mut num = 0;
    _create_rand_num(&mut num);
    loop {
        println!("请输入你猜的数字:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("请正确输入!");
        let input: i32 = input.trim().parse().expect("请 输入数字!");
        let mut hint = String::new();
        _check_num(input, num, &mut hint);
        println!("{}\n", hint);
        if hint == "你猜对了!" {
            break;
        }
    }
    println!("游戏结束!");
}

// 创建一个全局变量
// lazy_static! {
//     static ref GLOBAL_DATA: Mutex<Vec<i32>> = Mutex::new(vec![]);
// }

fn main() {
    // let handle = thread::spawn(|| {
    //     guess_num();
    // });
    // println!("Hello, world!");
    // // guess_num();
    // let mut arr = [1, 5, 3, 2, 4];
    // bubble_sort(&mut arr);
    // println!("{:?}", arr);
    // let mut arr = [1, 5, 3, 2, 4];
    // quick_sort(&mut arr);
    // println!("{:?}", arr);
    //
    //
    // // 等待新线程完成
    // handle.join().unwrap();

    let mut game = Game::new();
    game.run();
    // 用户输入任意 关闭
    println!("输入任意字符关闭...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("请正确输入!");
    // 输出彩色的文字在控制台

    // let mut data = GLOBAL_DATA.lock().unwrap();
    // data.push(1);
    // data.push(2);
    // println!("{:?}", data);
}
