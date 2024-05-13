///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_21
///
/// 结构体 struct
/// 一种复合类型，用于定义具有固定字段的数据结构，
/// 每个字段都有自己的名称和类型，可以是不同的类型
///

// 1 . 定义
#[derive(Debug)] // 为了 print
struct ShenYe {
    name: String,
    age: i32,
}

fn main() {
    // 2. 初始化
    let shenye1 = ShenYe {
        name: "深夜飚键盘".to_string(),
        age: 35,
    };
    println!("{:#?}", shenye1);
    // 3. 更新
    let shenye2 = ShenYe { age: 18, ..shenye1 };
    println!("{:#?}", shenye2);
    // 4. 访问字段
    println!("4.name = {} , age = {}", shenye2.name, shenye2.age);
    // 5. 可变结构体
    let mut shenye5 = ShenYe {
        name: "shenyekey".to_string(),
        age:18,
    };
    println!("5.1修改前 {:#?}", shenye5);
    shenye5.name = "深夜飚键盘".to_string();
    println!("5.2修改后 {:#?}", shenye5);

    // 6. 模式匹配
    match &shenye5 {

        ShenYe { name, age } => {
            println!("6. name = {} , age = {} ",name ,age);
        }
        
    }

    // 7. 所有权：结构体字段的所有权，属于结构体实例，当结构体销毁时，其字段
    // 占用的资源也将被释放
    // 8. impl 方法，结构体方法，定义结构体操作函数，后续 impl 展开

}
