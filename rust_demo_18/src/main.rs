///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_18
///
/// Rust 中的引用
/// 由于所有权可以直接让值发生转移，导致原始变量变得无效
/// 使用 引用，最为对值的非拥有情况下，可读可写的访问
/// 引用允许在不转移所有权的情况下，共享和操作数据

fn main() {
    // println!("Hello, world!");
    // 有几种引用，一起来看下吧

    // 不可变引用，对不可变变量的引用，格式 &变量名
    let s1 = String::from("深夜飚键盘");

    // 1. 同一时刻，可以任意数量的不可变引用指向同一个值
    let s2 = &s1;
    let s3 = &s1;
    println!("s2 = {} , s3 = {}",s2,s3);

    // 2. 不能同时存在对值的可变引用（等会会说说）
    // error : cannot borrow `s1` as mutable, as it is not declared as mutable
    // let s4 = &mut s1;

    // 3. 不能同时，修改不可变引用的值
    // error : cannot borrow `*s2` as mutable, as it is behind a `&` reference
    // let s5 = s2.push_str("string");

    // 第二种，可变引用，可变变量的引用 格式 &mut 变量名
    let mut m = String::from("深夜飚键盘");
    // 对值的独占，可写访问，同一时刻，只能存在一个可变引用，且不能同时存在其他任何引用
    // 简单点说，可变引用，要么只读，要么只写
    let m1 = &m; // 不可变引用
    let m2 = &m; // 不可变引用
    // let m3 = &mut m; // 可变引用
    // error cannot borrow `m` as mutable because it is also borrowed as immutable mutable borrow occurs here
    // println!("m1={},m2={},m3={}",m1,m2,m3);

    // 第三种，slice 引用，允许引用集合部分数据，可以不引用整个集合
    let arr = [1,2,3,4,5];
    // 只引用 1,2,3 值
    let arr1 = &arr[0..3];
    for a in arr1 {
        print!("a={a},");
    }

    // 第四种，垂悬引用，不展开了，遇到后，也会报错，防止悬挂指针的一些操作

    // 总结: 在任何给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用

}
