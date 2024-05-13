///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_23
///
/// 方法 impl
/// 给一种类型，如结构体和枚举，定义方法操作函数
/// 两种类型
/// 实例方法
/// 关联函数

//1 结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
// 和结构体名称一致
impl Person {
    // 关联函数（静态方法）
    // 这里返回值，是 结构体
    fn new(name: String, age: i32) -> Person {
        Person { name, age }
    }

    // 实例方法
    // 定义啦一个 说的函数
    fn say(&self) {
        println!("my name is {} , age {} ", &self.name, &self.age);
    }
}

// 2 枚举
#[derive(Debug)]
enum Month {
    Day1(i32),
    Day2(i32),
}

// 定义枚举方法
// 这里和枚举的名称也一样
impl Month {
    // 定义实例方法
    fn out(&self) {
        // 打印下他自己
        println!("enum hour : {:?}", &self);
    }
}

fn main() {
    // println!("Hello, world!");
    // 1. 结构体测试
    // 实例化
    let shenye = Person::new("深夜飚键盘".to_string(), 18);
    // 调用实例方法
    shenye.say();

    // 2. 枚举测试
    // 实例化
    let day1 = Month::Day1(18);
    day1.out();
}
