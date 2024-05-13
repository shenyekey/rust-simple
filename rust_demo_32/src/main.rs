///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_32
///
/// rust 生命周期
/// 表示一个值在内存中的有效时间；
/// 生命周期特制，引用所指向的数据的有效时间

// 1. 隐式 生命周期（Rust根据上下文自动推断）

fn print_name(name:&str){
    println!("{}" , name);
}

#[derive(Debug)]
struct Person {
    name:String,
}

// 2. 显示生命周期 
// 写法就是这样 '标记 ，'a ，a 为标记名称
fn demo2<'a,'b>(a:&'a str, b:&'b i32) -> &'a str {
    // 引用
    if a.len() as i32 > *b {
        "a.len > b"
    }else{
        "a.len <= b"
    }
}

// 3. 泛型
// a的生命周期，要比 demo3 长，demo3 才能用
struct demo3<'a,T> {
    a:&'a T,
    b:T,
}

// 4. 省略规则
// 省略规则也是隐式声明周期推断策略的一部分，专注于简化函数签名的生命周期标注

fn demo4(a:&str) -> &str {
    if a.len() > 5 {
        "hello rust"
    }else{
        "hello world"
    }
}

fn main() {
    // println!("Hello, world!");
    // 1. 
    let person = Person {
        name:String::from("深夜飚键盘"),
    };
    // 打印 name 自动推断 &person.name 的生命周期
    print_name(&person.name);

    // 2
    let s1 = "深夜飚键盘".to_string();
    let s2 = 3;
    println!("{}",demo2(&s1,&s2));

    // 4
    println!("{}",demo4(&s1));

}
