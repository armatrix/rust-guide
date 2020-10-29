fn main() {
    let str1 = "aaron";
    println!("str1: {}", str1);

    let mut str2 = String::from("anderson");
    str2.push_str(" matrix");
    println!("str2: {}", str2);

    // 堆上的数据被复制
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 这里是在栈上，编译时已知大小的类型被整个存储在栈上
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    // 这里已经没有s了， borrow of moved value: `s`
    // println!("s: {}",s);
    let x = 5;
    makes_copy(x);

    let s1 = String::from("hello"); //stack
    let s2 = "world"; // heap
    let s3 = s1;
    let s4 = s2;
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    let (s2, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}
