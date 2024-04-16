use rand::Rng;
use std::io;

fn main() {
    println!("Hello, world!");
    // 猜一猜数字，小游戏
    // 1. 先安装 rand 库

    // 2. 随机生成一个数字，用来猜这个数字
    let guess_num = rand::thread_rng().gen_range(1..100);
    println!("猜一猜，这个数字是多少啦？");

    // 3. 输入一个数字，看猜对了没
    // 增加可玩性1，没猜对，重新输入
    // 增加可玩性2，没猜对，给提示词
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read error");
        // 输入转为数字
        let input_num = input.trim().parse::<i32>().ok().expect("input not number");
        // 判断
        if guess_num == input_num {
            println!("猜对啦！猜对啦！，guess_num = {guess_num} , input_num = {input_num}");
            // 猜对，结束循环
            break;
        } else {
            // println!("猜错啦！");
            if input_num > guess_num {
                // 猜大提示
                println!("猜大啦！");
            }else{
                // 猜小提示
                println!("猜小啦！");
            }
        }
    }
    // 结束游戏提示
    println!("Game Over , 深夜飚键盘")

}
