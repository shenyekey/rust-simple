///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_24
///
/// 特征方法 trait impl
///
/// trait 以一种抽象的方式定义共同行为（函数），类似于其他语言的接口或抽象类
/// 允许不同类型实现相同的函数，实现类型间的通用
///
///

// 结构体实例

// 定义 trait 方法
trait Animal {
    // 1. 仅仅声明 方法签名
    fn say(&self);

    // 2. 声明方法签名 + 默认实现
    fn eat(&self, name: String) {
        println!("eat {} ", name);
    }
}

// 2. 定义 狗，包含狗-昵称
struct Dog {
    name: String,
    nick: String,
}

// 实现 trait 方法
impl Animal for Dog {
    fn say(&self) {
        println!("{}({}) say wang wang wang", self.nick, self.name);
    }
    // 这里没有实现 eat 方法
}

// 3. 再定义一个猫，包含 猫，年龄
struct Cat {
    name: String,
    age: i32,
}

// 实现 trait 方法
impl Animal for Cat {
    fn say(&self) {
        println!("miao , my name is {} , age {}", self.name, self.age);
    }
}

fn main() {
    // println!("Hello, world!");
    // 创建狗实例
    let dog = Dog {
        name: "中华田园犬".to_string(),
        nick: "旺财".to_string(),
    };
    dog.say();
    dog.eat("骨头".to_string());

    // 创建猫实例
    let cat = Cat {
        name: "橘猫".to_string(),
        age: 3,
    };
    cat.say();
    cat.eat("小鱼儿".to_string());

    // 使用 trait 定义了通用操作函数

    // 3. 特征方法作为函数参数
    // 小猫叫3遍
    for i in (0..3) {
        // 实现橘猫叫三遍,把橘猫引用参数传进去
        who_say(&cat);
    }

    // 4. 特征方法作为函数返回值
    // 创建3只小狗
    let dogs = ["小黄","旺财","小黑"];
    for nick in dogs {
        let dog = create_dog(nick.to_string());
        dog.say();
    }

}

// 作为函数参数
fn who_say(animal:&impl Animal){
    animal.say();
}

// 作为函数返回值
fn create_dog(nick:String) -> impl Animal {
    Dog {
        name: "中华田园犬".to_string(),
        nick:nick,
    }
}

