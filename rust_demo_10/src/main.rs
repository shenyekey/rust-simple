
// 函数定义

// 有参数函数,必须声明参数类型
fn add(x : i32) {
    println!("x = {x}");
}

// 有返回值函数，必须声明函数放回值类型
fn add1(x:i32 , y:f64) -> f64 {
    let z = x as f64 + y;
    println!("z = {z}");
    return z;
}

// 表达式写法，注意，表达式写法语句没有分号
fn add2(x:i32 ,y:f64) -> f64 {
    x as f64 + y
}

fn main() {
    add(1);
    add1(1, 2.0);
    let z = add2(1, 2.0);
    println!("z = {z}");
}
