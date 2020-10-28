fn main() {
    println!("Hello, world!");
    my_print(12);

    // 这里的x实际上不能向下传递，注意作用域问题,
    // let x = 5;
    // TODO 这里什么时候被回收♻️呢？
    let _x = 5;

    let y = {
        let x = 3;
        // 注意这里是表达式，没有分号
        x + 1
    };

    println!("The value of y is: {}", y);

    my_print(adder(1))
}

fn my_print(num: i32) {
    println!("{}", num)
}

fn adder(num: i32) -> i32 {
    num + 1
}
