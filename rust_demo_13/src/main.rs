// 循环，loop , while , for

// 1. loop

fn loop_1() {
    loop {
        print!("I' m loop");
    }
}

fn loop_2() {
    let mut i = 0;
    // break 返回值的 loop
    let res = loop {
        i += 1;
        println!("i = {i}");
        if i == 5 {
            break i + 1;
        }
    };
    println!("res = {res}");
}

// loop3 , loop 标签，loop 循环嵌套时，用于标记循环搭配 break 和 continue
// 进行使用
fn loop_3() {
    let mut a = 0;
    'a_loop: loop {
        let mut b = 5;
        // 嵌套循环，要结束整个循环语句
        loop {
            if b == a {
                // 这里使用标签，指明是那个loop
                break 'a_loop;
            }
            b -= 1;
        }
        // 这里测试 a 是否会变化
        a += 1;
    }
    // 如果 a 不变化则说明
    println!("a = {a}");
}

// while 循环
fn while_1() {
    // 常规写法
    let arr = [1, 2, 3, 4];
    let mut i = 0;
    while i < arr.len() {
        println!("value = {}", arr[i]);
        i += 1;
    }
}

// for 循环
fn for_1() {
    // for in 写法
    let arr = [1, 2, 3, 4];
    for value in arr {
        println!("value = {value}");
    }
}

// for range 循环
fn for_2() {
    // for range
    let arr = [1, 2, 3, 4];
    // 这里只去，3和4
    for i in (2..4).rev() {
        println!("for_2 value = {}", arr[i]);
    }

    for i in (0..3).by_ref() {
        println!("for 2 v = {}", arr[i]);
    }
}

fn main() {
    println!("Hello, world!");
    // loop_1();
    // loop_2();
    // loop_3();
    // while_1();
    // for_1();
    for_2();
}
