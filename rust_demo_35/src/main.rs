// 模块 
// create 包下，可以有多个模块
// 使用模块可以将包中的代码按照功能进行划分，增加可读性和易用性

// 1. 同模块使用
// 定义啦一个 模块 utils , 里面有个函数 sum ，并用 pub 公开，对外可访问
// mod utils {
//     pub fn sum(a:i32,b:i32) {
//         println!("{}",a+b);
//     }
// }

// use self::utils::sum;


// 2. 跨文件模块使用,（通常这样用）
mod utils;
// 有点像有些语言的 namespace
// 引入具体函数
// use crate::utils::sum;

// 引入全部函数
// use crate::utils::*;

// 引入部分函数
// use crate::utils::{sum1};

// 别名
// use crate::utils::{sum1 as add};

// 直接用 utils

// 3. 文件夹形式实现
// 用法和2一样
mod util1;

use util1::sum1;



fn main() {
    // println!("Hello, world!");
    utils::sum(1, 2);

    sum1(2,3);
}
