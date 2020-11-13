fn main() {
    // let _v: Vec<i32> = Vec::new();
    // 系统的默认字面量类型是i32
    let v = vec![1, 2, 3, 10, 6, 5, 4];
    // cannot borrow `v` as mutable, as it is not declared as mutable
    // v.push(2);
    println!("v: {:?}", v);

    {
        let mut v1 = vec![1, 2, 3];
        v1.push(5);
        println!("v1: {:?}", v1)
    }
    // v1 已经离开了作用域
    // println!("v1: {:?}", v1);

    let third: &i32 = &v[2];
    println!("third: {:?}", third);

    // todo 如何获取这里的参数，在下面进行引用
    match v.get(10) {
        Some(third) => println!("third: {:?}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("value: {:?}", i)
    }

    let mut v1 = vec![1, 2, 3, 10, 6, 5, 4];
    for i in &mut v1 {
        *i += 10;
    }
    for i in &v1 {
        println!("v1 value: {:?}", i)
    }

    // 假设我们有一个always run的系统，它持续的采集各种数据源的信息，并尝试将这些信息归档为某个实体，或是待寻找的实体。
    // 在起初的时候我们只知道某一部分的信息
    let mut user_rows = vec![
        User::Name(String::from("aaron")),
        User::Addr(String::from("china")),
        User::Age(12.2),
    ];
    println!("users value: {:?}", user_rows);

    match user_rows.get(1) {
        Some(first) => println!("user rows first: {:?}", first),
        None => println!("There is no first element."),
    }
    let some_user = &user_rows[2];

    let user = user_rows.pop();
    println!("pop user {:?}", user);
    user_rows.push(User::Name(String::from("neo")));
    println!("user rows after push: {:?}", user_rows);
    // println!("some_user: {:?}", some_user);
}

// set some default value
#[derive(Debug)]
enum User {
    Name(String),
    Age(f32),
    Addr(String),
}
