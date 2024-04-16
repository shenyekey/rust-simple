fn main() {
    println!("Hello, world!");
    // if 表达式
    let a = 5;
    if a < 5 {
        println!("a < 5");
    }
    // if else
    if a < 5 {
        println!("a < 5")
    } else {
        println!("a >=5")
    }
    // if else if 
    if a < 5 {
        println!("a<5");
    } else if a > 5 {
        println!("a>5");
    }else {
        println!("a=5");
    }

    // if 表达式写法，类似于 python 写法
    // 注意 if 和 else 返回值类型一致
    let b = if a < 5 { "a<5"} else { "a>=5"};
    println!("b = {b}");




}
