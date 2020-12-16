fn main() {
    let p1 = Person {
        name: String::from("p1"),
    };

    let p2 = Person {
        name: String::from("p2"),
    };

    let _p3 = Person {
        name: String::from("p3"),
    };

    let _p4 = Person {
        name: String::from("p4"),
    };

    let _p5 = Person {
        name: String::from("p5"),
    };
    drop(_p3);
    println!("user {:?}, {:?} has been created.", p1, p2)
}

#[derive(Debug)]
struct Person {
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("name: {} are leaving...", self.name)
    }
}
