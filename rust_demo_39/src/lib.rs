///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_39
///
/// 集成测试 
/// 创建的是一个 lib 包 , 因为集成测试只支持 lib 包的对外公开 API的测试
/// bin 包，不支持集成测试
/// 实现，需要在 src 同级目录创建 tests 文件夹，每个模块不需要标注 #[cfg(test)]
/// 



// 这里有一个 add 方法
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 2. 添加一个 utils 模块，对外的
// 这里 pub utils 对外公开
// 这里需要声明对外公开
pub mod utils;

// 3. tests 模块添加公共模块，比如执行初始化操作

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 首先执行的内部的 单元测试 
        // test tests::it_works ... ok
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 总结一下
// 测试函数 #[test] 标记
// 单元测试 src 目录下，使用 #[cfg(test)] 标记测试模块
// 集成测试 src 同级 tests 目录下 ，可不用 #[cfg(test)] 标记
// 集成测试 tests 目录下，也可以多模块实现 mod ，用法一致
