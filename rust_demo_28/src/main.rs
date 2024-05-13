///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_28
///
/// 宏 macro
/// 宏 是 rust 中一种特殊类型的函数，用于扩展代码
/// 在编译时，生成和修改代码的机制

fn main() {
    // 举几个例常用的宏
    // 1. println! 就是标准库的预定义宏，用于输出一行文本
    println!("Hello, world!");

    // 2. format! 标准库的宏，用于格式化字符串
    println!("{}",format!("{}","深夜飚键盘"));

    // 3. vec! 快速创建 vector 的便捷宏
    let v1 = vec!["123","456"];
    println!("{:?}",v1);

    // 4. 运行时程序错误宏， panic!
    let a = 10;
    // if a == 10 {
    //     panic!(" a is 10")
    // }

    // 5. 单元测试断言宏
    assert!(true);
    assert_eq!(1+1,2);

    // 6. 字符串组合宏
    let s6= concat!("shenyekey","深夜飚键盘");
    println!("{}",s6);

    // 7. 声明式宏，自定义
    macro_rules! shenyekey {
        () => {
            println!("深夜飚键盘")
        };
    }
    shenyekey!();

    // 8. 类函数宏，过程宏 #[derive(Debug)]
    #[derive(Debug)]
    struct Person {
        name:String,
    }
    println!("{:?}",Person{
        name:"深夜飚键盘".to_string(),
    })

}
