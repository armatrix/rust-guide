// 明确包的定义，访问修饰符的作用域，rust中定义结构体的默认访问级别，对包的引用路径在不同场景的使用,enum的不同
#![allow(unused)]
fn main() {
    fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist")
        }

        fn seat_at_table() {
            println!("seat_at_table")
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order")
        }

        fn server_order() {
            println!("server_order")
        }

        fn take_payment() {
            println!("take_payment")
        }
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 相对路径的引用
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("my toast");
    meal.toast = String::from("something else");
    // meal.seasonal_fruit = String::from("it cant be change.");
    println!("I'd like {} plz.", meal.toast);
    let order = back_of_house::Appetizer::Soup;

    // keyword: use
    use back_of_house::Breakfast;
    let another_meal = Breakfast::summer("new toast");
}
