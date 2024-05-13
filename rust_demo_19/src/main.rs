///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_19
///
/// 用引用 再写下 猜一猜数字小游戏啦
/// 回顾
/// 1. 生成一个数字谜底 guess_num
/// 2. 用户输入一个数字 input_num
/// 3. 策略逻辑，判断是否猜对
/// 3.1 猜对，输出恭喜词，结束游戏
/// 3.2 猜错，输出提示词，重新输入

use rand::Rng;
use std::io;


// 1. 第一步 生成一个数字谜底 guess_num

fn create(guess_num:&mut i32){
    // 指针值改变
    *guess_num = rand::thread_rng().gen_range(1..100);
}

//  2. 用户输入一个数字 input_num
fn input(input_num:&mut i32){
    // 输入内容
    let mut num_str = String::new();
    // 键盘输入
    io::stdin().read_line(&mut num_str).expect("input error");
    // 字符串转数字
    *input_num = num_str.trim().parse::<i32>().ok().expect("input not number");
}


/// 3. 策略逻辑，判断是否猜对
/// 定义输入 guess_num , 不可变引用
/// 定义输入 input_num , 不可变引用
/// 定义输入 word , 提示词，可变引用
/// 定义返回值 bool ，是否猜对
fn exec_once(guess_num:&i32 , input_num:&i32,word:&mut String) -> bool {
    if guess_num == input_num {
        *word = format!("猜对了，猜对了， {} , {} ",guess_num,input_num);
        // 返回值
        true
    }else{
        // 提示词
        if input_num > guess_num {
            *word = "猜大了".to_string();
        }else{
            *word = "猜小了".to_string();
        }
        false
    }
}


fn main() {
    // 1. 谜底数字
    let mut guess_num = 0;
    create(&mut guess_num);
    // 2. 输入数字
    let mut input_num = 0;
    // 3. 提示词
    let mut word = String::from("猜一猜，这个数字是多少呢？");
    // 4. 执行逻辑
    loop {
        // 输出提示词
        println!("{}",word);
        // 重新输入,改变 input_num 的值
        input(&mut input_num);
        // 执行逻辑
        if exec_once(&guess_num, &input_num, &mut word) {
            // 返回 true ， 猜对了
            break;
        }
    }
    // 5. 游戏状态结束
    println!("{word}");
}
