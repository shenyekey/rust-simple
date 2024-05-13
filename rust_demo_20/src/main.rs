///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_20
///
/// 枚举 enum
/// Rust中的复合类型，运行定义一组相关的值，成员，可表示一种状态
/// 

// 1. 常规-无数据枚举
#[derive(Debug)]
enum Month {
    Day1,
    Day2,
}

// 2. 带有元组的数据的枚举
// 比如 Day1 天的几点
#[derive(Debug)] // 这个是为了可以 print
enum Month2 {
    Day1(i32),
    Day2(i32),
}

// 3. 带有结构体数据的枚举（没关系，结构体下篇讲）
// 比如 Day1 天的几点几分
#[derive(Debug)]
enum Month3{
    Day1 {hour:i32 , minute:i32},
    Day2 {hour:i32 , minute:i32},
}

fn main() {
    println!("1.{:?}",Month::Day1);
    println!("2.{:?}",Month2::Day1(8));
    println!("3.{:?}",Month3::Day1 { hour: 8, minute: 30 });

    // 怎么用，使用模式匹配，先看下
    // 定义了 day2天 9 点 30 分
    let day2 = Month3::Day2 { hour: 9, minute: 30 };
    match day2 {
        Month3::Day1 { hour, minute } => {
            println!("day 1 hour : {} , minute : {}",hour,minute);
        }
        Month3::Day2 { hour, minute } => {
            println!("day 2 hour : {} , minute : {}",hour,minute);
        }
    }
    // 枚举方法，可以为枚举定义操作函数，后续 impl方法展开
    

}
