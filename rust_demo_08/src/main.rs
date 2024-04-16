fn main() {
    // int 类型，默认 int32
    let i = 100;
    let i1:i64 = 100;
    println!("i = {i} , i1 = {i1}");
    // float 类型
    let f = 10.1;
    let f1:f32 = 5.5;
    println!("f = {f} , f1 = {f1}");
    // bool 类型
    let b = true;
    let b1 : bool = false;
    println!("b = {b} , b1={b1}");
    // 字符类型
    let c = '😄';
    let c1:char = '🙂';
    println!("c={c} , c1={c1}");
    // 复合类型 
    // 元组
    let t = (1,2,3);
    let (t1,t2,t3) = t;
    println!("{t1} {t2} {t3}");
    // 数组，定义 [类型;长度]
    let arr = [1,2,3];
    let arr1 :[i64;4] = [1,2,3,4];
    let arr2 = [10.1;5];
    // 字符串
    let s = "深夜彪键盘";
    let s1:&str = "我就是";
    println!("s = {s} , s1 = {s1}");


}
