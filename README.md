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

使用\#[derive(Debug)] 注解来增加注解来派生 `Debug` trait

 {:?}使用调试格式

```rust
# 这个也很类似go
p1.distance(&p2);
(&p1).distance(&p2);
```

类似String::from是关联函数*associated functions*，不是方法，使用:: 来调用

### Enums

里面的类型是常量吗？

从入门的代码我们可以看到，这里的枚举是在和struct进行比较，可很显然的是，我们想通过枚举来对某些值进行限定，而将struct认为是更大的一组抽象。从扩展的角度，似乎用enums并不合适，我们无法对未来可能存在的变化作出准确的预测，反之，这里的枚举似乎更适合来用作作为下边界。如假定我们要对系统的权限做明确的划分，有若干个等级，首先要具备哪些基本的条件才能触发一定的等级，这里用预先定义的枚举来管理权限再合适不过了。同样的，对其属性的抽象可以通过该种方法，那么对其可能具备的“最低”行为，我们也可以通过这种“下边界”的方法来定义。

对部分边界的考量种，如go中采用给特定类型以默认值的方式来处理，在rust中则通过限制空值来增加代码的安全性

match将一个值与一系列的模式进行匹配

`let if` 让我们只处理1与非1的情况

### Packages, crates and modules

 A *package* is one or more crates that provide a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.

一个包中必须包含一个crate，包含的crate中最多只能有一个library crates，通过Cargo.toml去描述如何构建这些crate

通用约定：*src/main.rs*  就是一个与包同名的二进制 crate 的 crate 根，类似的，库的crate根在 *src/lib.rs*下

命名路径（paths） use，pub，as，外部包，glob运算符

注意这里模块包含关系，这里类似文件系统的自上而下，和go中不同

对crate的寻址也包含绝对路径和相对路径两种，一种以crate开头，一种以self、super或当前模块的标识开头

Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的，go中使用开头是大小写来标识，尽管使用的pub来让结构体变成公有的，但内部字段仍然是私有的，但在enum中一旦声明，所有的都变成了公有的

use, use as, pub use

```rust
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::collections::*;
```

mod对包进行进一步的拆分

Collections

集合的数据存储在堆上，数量可以在运行时增长或缩小。

vector在内存中彼此相邻的排列，vector只能存储相同类型的值，同样的，离开作用域即销毁，对vector的访问可以通过索引和get方法。

对vector进行引用之后，不可以在对vector进行push等会对内存进行重新分配的操作，



## TODO

宏调用不进行回收吗

类单元结构体

```rust
impl Rectangle {
    fn destory(self) {}
}
```

结构体的赋值可以在有默认值的同时通过后面的赋值来指定吗

vector同go的sli进行对比