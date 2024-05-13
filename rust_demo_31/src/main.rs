///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_31
///
/// 
/// 错误处理 error
/// rust 的错误机制特别严谨，确保程序面对错误时，能够安全的运行
/// 
///  1. 可恢复错误，程序运行中的可接受错误，比如输入错误参数
///  2. 不可恢复错误，程序系统性错误，如 数组越界
/// 
/// 

// 举个例子 除法

fn divide(a:i32,b:i32) {
    println!(" a / b = {}" , a/b);
}


fn divide2(a:i32,b:i32) {
    if b==0{
        // 宏
        panic!("b is 0 ")
    }
    println!(" a / b = {}" , a/b);
}

// 主动处理异常，使用 Result 
// 这样就主动处理啦异常
fn divide3(a:i32,b:i32) -> Result<i32,String> {
    if b==0 {
         // 提示给使用者，或者返回默认值
        return Err("b is 0".to_string());
    }
    return Ok(a/b);
}


fn main() {
    // println!("Hello, world!");
    // 1. 没限制，触发，程序就崩了
    // divide(1,0);

    // 2. 主动触发，也崩了； 
    // divide2(1,0);

    // 3. 主动抛出，程序不崩
    let result = divide3(1,1);
    let value = match result {
        Ok(value) => value,
        Err(error) => {
            println!("{}",error);
            // 返回默认值
            -1
        }
    };
    println!("{}",value)

}
