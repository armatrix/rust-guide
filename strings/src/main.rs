fn main() {
    let s = String::new();
    println!("s: {:?}", s);

    let name = "aaron";
    let mut s = name.to_string();
    println!("s: {:?}", s);

    s.push_str(name);
    println!("s: {:?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);

    let mut s3 = String::from("lo");
    // 这里是char
    s3.push('l');
    println!("s3: {}", s3);

    let s4 = String::from("hello, ");
    let s5 = String::from("world");
    // 这个样子是不行的
    // let s6 = &s4 + s5;
    // 这个里面将 &String 强转（coerced）成&str，这里使用到一叫做解引用的强制多态（deref coercion），简单的可以将其理解为其把&s2转换为&s2[..]
    let s6 = s4 + &s5;
    println!("s6: {}", s6);
    // println!("s4 is {}", s4);
    println!("s5: {}", s5);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {:?}", s);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    let answer = &hello[0..2];
    println!("answer: {:?}", answer);

    for c in hello.chars() {
        println!("c: {:?}", c);
    }

     for b in hello.bytes() {
        println!("b: {:?}", b);
    }
}
