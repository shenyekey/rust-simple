
// 使用标准库

use std::io;

fn main() {
    // 声明 a
    let mut a = String::new();
    // 输入 a
    io::stdin().read_line(&mut a).expect("read line error");
    // 转换 a
    let b = a.trim().parse::<i32>().ok().expect("a not number");
    // 求平方
    let c = b * b;
    // 输入 c
    println!("output {c}");
}
