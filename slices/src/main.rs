fn main() {
    let s = String::from("hello, world");

    let slice = &s[0..2];
    println!("slice: {}", slice);
    let slice = &s[..2];
    println!("slice: {}", slice);

    let word = first_word(&s);
    println!("word: {}", word);
}

fn first_word(s: &String) -> &str {
    let word_in_bytes = s.as_bytes();

    for (i, &item) in word_in_bytes.iter().enumerate() {
        println!("inner for s: {}, i: {}", s, i);

        if item == b',' {
            println!("inner if s: {}, i: {}", s, i);
            // 这里修改了s
            return &s[0..i];
        }
    }
    // TODO注意这里是不会输出的，似乎这里不是自上而下的
    println!("outter s: {}", s);
    &s[..]
}
