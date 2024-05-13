///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_27
///
/// 泛型 generics
/// 偏于理解
/// 使用 大写的 T ,U , K,V, 作为类型参数或返回值，来定义函数，结构体等
/// 编译时，被具体的类型转换为特定代码
///

// 1. 函数
// 如 求两个数之和
// i32
fn add1(a: i32, b: i32) -> i32 {
    a + b
}
// 那 f64 呢，是不是要再写一遍
fn add2(a: f64, b: f64) -> f64 {
    a + b
}
// 使用泛型，把类型参数,
// std::ops::Add<Output = T> 时，rust 严格要求要输入类型可以进行 + 操作
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 2. 结构体泛型
// 定义一个点结构体，有 x ，y 坐标点
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 3. 方法中泛型
impl<T> Point<T> {
    fn say_x(&self) -> &T {
        &self.x
    }
}

// 4. trait 特征，泛型关联类型
trait Animal {
    // 使用这个标记 type Option
    type Food;
    fn eat(&self) -> Option<Self::Food>;
}

struct Dog {
    name:String,
}

impl Animal for Dog {
    type Food = String;

    fn eat(&self) -> Option<Self::Food> {
        println!("{} eat",&self.name);
        // Some 配合 Option使用
        Some("eat".to_string())
    }
}


fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1.0, 2.1));
    println!("{:#?}", Point { x: 1, y: 1 });
    println!("{:#?}", Point { x: 1.2, y: 1.2 });
    // 3
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.say_x());
    // 4 dog
    let dog = Dog {
        name: "旺财".to_string(),
    };
    dog.eat();

    // 5 默认泛型类型
    struct Demo<T = i32> {
        value:T,
    }
    // 使用默认类型 
    let d1 = Demo {value: 100};
    // 显示指定其他类型
    let d2 :Demo<String> = Demo { value: String::from("Demo") };
    


}
