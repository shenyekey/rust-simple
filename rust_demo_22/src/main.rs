///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_22
///
/// 模式匹配 match
/// 一种编程机制，用于解构复合数据类型的值，并执行不同的分支代码
/// 复合数据类型有 枚举，结构体，数组，元组
///

// 这里用结构体说明

#[derive(Debug)]
struct ShenYe {
    name: String,
    age: i32,
}

fn main() {
    // 结构体
    let shenye = ShenYe {
        name: "深夜飚键盘".to_string(),
        age: 18,
    };
    println!("{:?}", shenye);
    // 1. match
    match &shenye {
        ShenYe { name, age } => {
            println!("1.match语法 name ={} age ={}", name, age);
        }
    }
    // 2. let 绑定
    let ShenYe { name: n, age: a } = &shenye;
    println!("2.let绑定 name ={} age ={}", n, a);

    // 3. if let
    if let ShenYe { name, age } = &shenye {
        println!("3.if let name ={} age ={}", name, age);
    }

    // 4. 守卫，bool类型的使用 match
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        match i {
            i if i % 2 == 0 => {
                println!("偶数: {}", i);
            }
            _ => println!("奇数：{}", i),
        }
    }

    // 枚举类型
    enum Month {
        Day1(i32),
        Day2(i32),
    }
    let month = Month::Day1(30);
    match month {
        Month::Day1(hour) => {
            println!("day1 hour {}",hour);
        },
        Month::Day2(hour)=>{
            println!("day2 hour {}",hour);
        }
    }
    // 元组 
    let tuple1 = (1,2,3);
    // if let
    if let (a,b,c) = tuple1 {
        println!("a = {} , b = {} , c = {}",a,b,c);
    }

    // 数组
    let arr2 = [1,2,3];
    if let [a,b,c] = arr2 {
        println!("a = {} , b = {} , c = {}",a,b,c);
    }
}
