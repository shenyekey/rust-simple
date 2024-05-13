///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_30
///
/// rust 中常用集合 
/// vector String HashMap
/// 
/// 


// 1. vector 动态数组，容量可变，存储连续内存中相同类型的元素

use std::collections::HashMap;

fn vector1(){

    // 1. 创建
    // 必须指明类型
    let mut v1:Vec<i32> = Vec::new();
    // 第二种方式 宏
    // let v2 = vec![1,2,3];
    // 2. 添加元素
    v1.push(1);
    v1.push(2);

    // 3. 遍历
    for i in &v1 {
        println!("{}",i);
    }

    // 4. 修改
    v1[0] = 100;

    // 获取单个值
    println!("{:?}",v1.get(0));
    println!("index=0 , {}",v1[0]);


}


// 2. String 字符串类型，用于存储 utf-8 的文本数据

fn string1(){
    // 新建
    let mut s1 = String::new();
    let mut s2 = String::from("深夜飚键盘");
    let mut s3 = "shenyekey".to_string();

    // 更新
    s1.push_str(" shenyekey");
    println!("{}",s1);

    // 拼接 format! 宏 和  concat! 宏均可

    // 取部分值
    println!("{}",&s2[0..3]);

    // 遍历
    for c in s2.chars() {
        println!("{}",c);
    }
}

// 3. hashmap: 键值对，基于哈希表的实现，KV泛型指定类型

fn hashmap1(){

    // 1. 创建
    let mut map : HashMap<String,i32> = HashMap::new();
    // 2. 新增
    map.insert("旺财".to_string(), 1);
    map.insert(String::from("花花"), 2);

    // 3. 获取
    let v = map.get("旺财");
    if let Some(v) = v{
        println!("旺财: {} year old",v);
    }

    // 4. 更新，同key覆盖
    map.insert("花花".to_string(), 5);
    println!("{:#?}",map);

    // 5. 遍历
    for (k,v) in &map {
        println!("{} , {} year old",k,v);
    }

    // 6. 修改-稍微复杂写
    // 大概意思就行，花花这个key ,进行修改，修改成多少，修改不成，默认填多少
    map.entry("花花".to_string()).and_modify(
        |year| *year = 10
    ).or_insert(0);

    println!("{:#?}",map);
}


fn main() {
    // println!("Hello, world!");
    // 1 
    // vector1();
    // 2
    // string1();
    // 3
    hashmap1();
}
