///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_37
///
/// 测试 test
/// 测试函数是用来验证非测试代码是否按照期望的方式运行
/// 1. 设置任何所需的数据或状态
/// 2. 执行测试代码
/// 3. 断言其结果是否是我们所期望的
/// 


// 定义个函数
fn add(x:i32 , y : i32) -> i32{
    println!(" x {x} , y {y}");
    x + y
}

fn main() {
    println!("Hello, world!");
}

// 测试函数
// #[test] 注解标记，表明这是一个测试函数，必须标记
// 可以单独运行
// 有了 #[test] 标记，实际代码在编译过程中，就会被忽略
#[test]
fn test_add(){
    // assert_eq! 这个宏是断言，左边值等于右边值得到预期
    assert_eq!(add(1,2),3);
}

// 结果说明
// 表明此函数测试通过
// test test_add ... ok  
// 表明 1 个通过，0个失败
// test result: ok. 1 passed; 0 failed;

#[test]
fn test_add1(){
    // 3 !=4
    // test test_add ... ok
    // test test_add1 ... FAILED
    assert_eq!(add(1,2),3);
}

// 命令执行 
// cargo test ，在测试模式下编译代码并运行生成的测试二进制文件，
// 并且是并发运行所有的测试，并截获测试过程中产生的输出

// 若不使用并发机制的运行测试用例，可以添加参数
// cargo test -- --test-threads=1

// 也可以单独执行某个测试函数
// 默认执行测试，是不显示输出的 println! 输出的

// 可以增加参数，显示 println! 输出
// cargo test -- --show-output
