#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn destory(self) {}

    fn can_hold(&self, other: &Rectangle)->bool{
        self.height>= other.height && self.width>=other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1: {:?}", rect1);
    println!(
        // todo 这里有啥用吗 debug这个
        "The area of the rectangle is {:?} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_ownership(rect1)
    );
    // 这里所有权是没有的哦，复习一下
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_ownership(rect1)
    // );

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };

    let area2 = rect2.area();
     println!(
        "The area of the {:?} is {} square pixels.",rect2,area2
    );

    let can_hold =  rect3.can_hold(&rect2);
    println!("{:?} can hold {:?} :{}",rect3,rect2,can_hold);

    // 在声明周期之外会释放，这样也可以
    rect2.destory();
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_ownership(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
