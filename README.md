# rust guide

### 基础概念

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



### 变量和基础数据类型

我们一直都说声明一个变量，变量变量顾名思义是可以变的，然而在rust中它是immutable的，作为新时代的语言，这样搞咩啊，需要用 `mut` 关键字来让其可以变化

使用const声明常量时需进行类型注解，当然也只能设为常量表达式，而不能通过函数调用来产生，或者说不能通过procedure来产生，其生命周期在整个程序中作用

string 默认也是UTF-8 编码，数字使用i32

```shell
# 本地依赖文档
cargo doc --open
```

每一个值都属于一个数据类型，两类数据类型子集：标量（scalar）和复合（compound）

标量：整型、浮点型、布尔类型和字符类型，数字之间可以用下划线隔开提高可读性

整型中arch对应的isize和usize跟随系统架构的位数，和go一样分别使用0b,0o,0x代表二进制、八进制和十六进制数

char类型占四个字节

两个原生的复合类型 元组（tuple）和数组（array）

### 函数

函数调用 是一个表达式

宏调用 是一个表达式

{} 是一个表达式

注意这里函数表达式在返回值中的写法，没有特殊的符号来注明，表达式中每个值都应该返回同种类型的值， 如下面的错误示例

```rust
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
```

### ownership

一些程序语言通过gc来回收内存，如go

一些需要手动管理，如c

rust采用第三种，所有权的方式

调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈

1. Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

Rust 在结尾的 `}` 处自动调用 `drop`

Rust 永远也不会自动创建数据的 “深拷贝”

任何 **自动** 的复制可以被认为对运行时性能影响较小

编译时已知大小的类型被整个存储在栈上

标量，能确定的内存布局的相关类型及其组合是可以用copy的

### 引用与借用

不能在拥有不可变引用的同时拥有可变引用，即在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用

数据的引用，编译器确保数据不会在其引用之前离开作用域，引用必须总是有效的

❌示例

```rust
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

```rust
# 基于这种也是❌的，和声明周期有关，todo
fn not_dangle(some_string: String) -> &String { 
    &some_string
} 
```

### slices

特别要注意返回值的问题，注意return的位置

这里和go很类似，go是对底层数组的引用，这里则是对String的引用

### Struct

在结构体上定义方法和关联函数来指定与结构体数据相关的行为

这个和go就比较像了

注意这里引用另外一个结构体的时候的写法，类似go中的拆包

tuple stuct，这里的black 和origin不是同一个类型，这个类似go的type

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

还有个没有任何字段的类单元结构体

>  **类单元结构体**（*unit-like structs*）因为它们类似于 `()`，即 unit 类型。类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用



生命周期确保结构体引用的数据有效性跟结构体本身保持一致，&str是不具备拥有所有权的，注意这里声明相关的结构体field需额外声明lifetimes





## TODO

宏调用不进行回收吗

类单元结构体