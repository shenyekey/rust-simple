///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_33
///
/// 工作区间 workspace 
/// 比如这个 rust_demo_33 文件夹就是一个工作区间
/// 工作区间为共享共同配置和配置
/// 工作区间内所有的 creates 可以共享
/// 几种文件夹结构
/// 
/// 第一种 
/// - workspace 
///     - crate_lib1  // lib crate
///     - crate_lib2
///     - src         // bin crate
///     - Cargo.toml  // 共享配置和依赖
///     - Cargo.lock
/// 
/// 第二种
/// - workspace
///     - crate_lib1
///     - crate_lib2  // lib crate
///     - crate_bin  // bin crate
///     - Cargo.toml // 共享配置和依赖
/// 
/// 第三种
/// - workspace 
///     - crates
///         - lib1
///         - lib2 // lib crate
///     - crate_bin  // bin crate
///     - Cargo.toml
/// 
/// 
/// 其实都大差不大，看自己工程需求，
/// 1，一个文件夹下，有 Cargo.toml 来共享配置和依赖
/// 2，然后使用 cargo new 创建 bin 和 lib 库
/// 3， 配置启动项或者对外调用项
/// 4，使用即可
/// 一个工作区间可以有多个 bin 和 多个 lib 
/// 
/// 这里我们演示下第二种
/// 
/// cargo run --bin main 指定运行那个 bin
/// 因为 这个 rust_demo_33 也是个 bin 
/// 其实可以删除 src 这个文件夹，只保留 main bin的
/// 
/// cargo build --package main_lib
/// 可以指定 编译那个 lib 
/// 
/// 工作区间可以安装自己的项目进行实现结构，非固定上述的三种

fn main() {
    println!("Hello, world!");
}
