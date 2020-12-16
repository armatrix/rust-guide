use std::ops::Deref;
fn main() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // *(y.deref())
    assert_eq!(5, *y);

    let name = MyBox::new(String::from("aaron"));
    say_hello(&name);
    // 没有解引用强制多态（deref coercions），行为发生在编译时，并不会产生运行时消耗
    say_hello(&(*name)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn say_hello(name: &str) {
    println!("hello, {}!", name)
}
