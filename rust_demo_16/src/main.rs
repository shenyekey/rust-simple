///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_16
///
/// Rust 作用域，一个代码块，程序执行到时，创建变量分配内存
/// 程序执行完毕，销毁变量释放内存
/// 例子：创建 s1 和 s2 ，debug 模式观察变量创建和释放过程
///
/// 我们打几个断点，debug 模型观察 

fn my() {
    // 创建 my statck ...9cc,此时 s2 分配了内存，继续
    let s2 = "hello rust";

    // 输出s2，执行到这里，s2 赋值了，继续，println 完毕，s2 被销毁了
    println!("{}", s2);
}

fn main() {
    // 可以看到程序执行到这里，给s1分配了内存，但还没赋值，继续
    let s1 = "深夜飚键盘";

    // main 的 stack 为 ...bee
    // 执行到这里，s1 语句执行完毕，s1 被赋值了，可以看到;进入my函数看看，回到main函数，my stack 被释放了
    my();

    // 继续，观察 s1，执行到 main } 下面的括号，s1 被销毁了
    // 继续 ，执行完毕，main stack 被销毁了
    // 输出 s1
    println!("{}", s1);
}
