///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_29
///
/// println! 宏，格式化例子
/// 支持多种格式说明服和参数

fn main() {
    // 1. 输出字符串
    println!("Hello, world!");

    // 2. 参数输出
    let name = "深夜飚键盘";
    println!("Hello {}",name);

    // 3. 输出结构体，配合 过程宏
    #[derive(Debug)]
    struct Person {
        name:String,
    }
    // {:#?}, 带格式化  {:?} 不带格式化
    println!("{:#?}",Person{
        name:"深夜飚键盘".to_string(),
    });

    // 4. 数值格式化
    let num = 12345;
    println!("{:0>8}",num); // 长度为8，不够补0

    // 5. 浮点数
    let num1 = 12.3456;
    println!("{:.2}",num1); // 取小数点2位

    // 6. 输出指针地址
    let ptr = &name; // 引用上面的 name
    println!("{:p}",ptr);

    // 7. 命名参数,没有定义 a 和 b 变量
    println!("{a} , {b}",a="hello",b="深夜飚键盘");


}
