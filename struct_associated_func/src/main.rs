// 关联函数
fn main() {
    let sq = Rectangle::square(3);
    println!("{:?}",sq)
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}