// todo 这是一个非常糟糕的例子
use std::cell::RefCell;

fn main() {
    let admin_tag = String::from("admin");
    let user_tag = String::from("user");
    let p = Person{
        name: String::from("aaron"),
        tag: RefCell::new(vec![admin_tag,user_tag])
    };

    let mut m = LimitTracker::new(&p, 100);
    m.set_value(10);
    m.set_value(100);
    m.set_value(90);
}

pub struct Person {
    name: String,
    tag: RefCell<Vec<String>>,
}

impl Messager for Person {
    fn send(&self, msg: &str){
        println!("name: {:?}, msg: {:?}",self.name, msg);

        // thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:27:18
        let mut _borrowed = self.tag.borrow_mut();
        self.tag.borrow_mut().push(String::from(msg));
        println!("name: {:?}, tag: {:?}",self.name, self.tag);
    }
}


pub trait Messager {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messager.send("some value/max gt 1.0 message")
        } else if percentage >= 0.9 {
            self.messager.send("some value/max in 0.9-1.0 message")
        } else {
            self.messager.send("some value/max lt 0.9 message")
        }
    }
}
