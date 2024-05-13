
# 编译产物

rust 的编译产物 统一在 target 目录下，目的为了隔离产物，保持源代码的干净整洁

## 可执行程序包 

- cargo build ，编译后产生 target 目录
- 1. debug , 调试或开发模式的目录
- 2. release 发布模式产生的产物目录
- 3. debug/build 存放构建脚本，自动复制
- 4. debug/deps 存放依赖项
- 5. debug/examples 存放示例代码
- 6. debug/incremental 增量编译缓存目录，增加编译速度
- 7. debug/demo_36 二进制可执行文件
(windows下为 demo_36.exe 可执行文件)
- 8. debug/demo_36.d 记录源代码编译的依赖关系，编译时自动生成
- 9. release 目录结构一致

## 库 包

- cargo build 产生 target 目录
- 目录一致
- 3. debug/build 存放构建脚本，自动复制
- 4. debug/deps 存放依赖项
- 5. debug/examples 存放示例代码
- 6. debug/incremental 增量编译缓存目录，增加编译速度
产物不一致
- 7. debug/demo_36_libs.rlib rust静态链接库，方便被
其他可执行文件或库引用

- 8. debug/demo_36_libs.d 记录源代码编译的依赖关系，编译时自动生成

##  库 包 静态库和动态库

先说说什么是静态库和动态库

- 静态库，第三方链接时，直接被嵌入编译到程序里；
- 动态库，第三方链接时，作为依赖项进行调用的；

rust 库包怎么编译静态库和动态库 

```
[lib]
# 静态库类型
crate-type = [ "staticlib" ]
```
编译产物，见 libdemo_36_libs.a ，后缀名为 *.a 

- macos : *.a
- linux : *.a 
- windows: *.lib


怎么编译动态库呢

```
[lib]
# 静态库类型
crate-type = [ "cdylib" ]
```
见编译产物，libdemo_36_libs.dylib ，后缀名为 *.dylib

- macos : *.dylib
- linux : *.so
- windows: *.dll


