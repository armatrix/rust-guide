fn main() {
    // if
    let num = 14;
    let cmp_num = 10;
    if num < cmp_num {
        println!("less than {}", cmp_num)
    }
    eprintln!("not less than {}", cmp_num);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter + 3;
        }
    };
    println!("result: {}", result);

    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("elem: {}", elem);
    }

    let mut sum = 0;
    for num in 0..100 {
        sum += num
    }
    println!("sum: {}", sum);

    let mut tally = 0;
    // 逆序
    for num in (0..100).rev() {
        tally += num
    }
    println!("tally: {}", tally);
}
