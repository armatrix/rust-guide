fn main() {
    // let x = 5;
    let _x = 5;
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    // shadowing, 这里每次使用let实际上
    let y = 1;
    let y = y * 2;
    let y = y * 2;
    println!("y: {}", y);

    let spaces = "  ";
    let spaces = spaces.len();
    // let spaces: usize = spaces.len();
    println!("spaces length: {}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("tuple y : {}", y);
    println!("value of index 2: {}", tup.2);

    // array 不可变长，这里对应的可变长是vector
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 这里不行，该类型[&str; 12] 没有实现std::fmt::Display
    // println!("months: {}", months);
    println!("months: {}", months[9]);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[3]: {}", arr[3]);
    let index = 10;
    println!("index: {},arr[index]: {}", index, arr[index]);
    // 上面出问题了，下面也不会执行了
    let zero = 0;
    let num = 3 / zero;
    println!("num: {}", num)
}
