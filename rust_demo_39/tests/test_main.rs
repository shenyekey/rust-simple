
use rust_demo_39;

use rust_demo_39::utils;

mod common;
use common::setup;

#[test]
fn test_add(){
    // 其次是执行的是集成测试 
    // test test_add ... ok
    assert_eq!(rust_demo_39::add(1, 2),3);
    // 后面还有个文档测试,这里就不展开了
    //  Doc-tests rust_demo_39
}

#[test]
fn test_add1(){
    setup();
    // 测试 utils::add1 函数
    assert_eq!(utils::add1(3, 4),7);
}