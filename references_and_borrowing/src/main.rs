fn main() {
    let s1 = String::from("hello");

    let s2 = String::from("world");
    let s1_len = calculate_length(&s1);
    let s2_len = calculate_length_without_passing_owner(s2);

    println!("The length of '{}' is {}.", s1, s1_len);
    // 这里的s2已经没了
    println!("The length of '{}' is {}.", "s2", s2_len);
    println!("s1: {}", s1);

    let mut s3 = String::from("hello");

    {
        let r1 = &mut s3;
        println!("r1: {}", r1);
    }

    let r2 = &mut s3;
    println!("r2: {}", r2);

    let r4 = &s3;
    let r5 = &s3;
    // cannot borrow `s3` as mutable because it is also borrowed as immutable
    // let r6 = &mut s3;
    // println!("r4: {}, r5: {}, r6: {}",r4,r5,r6);
    println!("r4: {}, r5: {}", r4, r5);
     // todo 这个和上面的宏调用不是不会对其进行改变吗， r4 r5 不是应该还在吗， 为啥还能引用
    // println!("r4: {}, r5: {}", r4, r5);
    let r7 = &mut s3;
    // println!("r4: {}, r5: {}", r4, r5);
    
    println!("r7: {}", r7)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_length_without_passing_owner(s: String) -> usize {
    let length = s.len();
    length
}
