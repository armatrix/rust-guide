# rust guide

**预编译静态类型**（*ahead-of-time compiled*）语言

rustc 进行编译，同其他的变异指令一样，`-o` 参数指定文件名称，如将demo文件编译成指定的名称

```shell
rustc main.rs -o hello_world
```

类似gofmt和其他编程语言的fmt工具，rustfmt提供了rust代码的标准格式化代码工具，rust中代码缩进为四个空格，和go不同的是，rust的语句需要用分号

代码包被称为 *crates*

注意Result中的枚举

Cargo 

```shell
# 构建项目,使用./target/debug/<des file> 运行 
cargo build 
# 编译并运行
cargo run
# 快速检查代码确保其可以编译，但并不产生可执行文件（这个有点意思，不到编译阶段如何能确保其可以编译成功，谁在做这个工作？）,感觉还是在编译
cargo check 
# 优化编译，在target/release 目录下
cargo build --release
```



我们一直都说声明一个变量，变量变量顾名思义是可以变的，然而在rust中它是不可以的，需要用 `mut` 关键字来让其可以变化，emmmm

string 默认也是UTF-8 编码，数字使用i32

```shell
# 本地依赖文档
cargo doc --open
```

