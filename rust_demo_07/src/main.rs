fn main() {
    // 声明一个不可变的变量 a
    let a = 10;
    println!("a = {a}");
    // 声明一个可变的变量 b
    let mut b = 10;
    b = 100;
    println!("b = {b}");
    // 声明一个常量
    const C:i32 = 2024;
    println!("C = {C}");
    // 不可变的变量可以被覆盖
    let a = "深夜飚键盘";
    println!("a = {a}");
}
