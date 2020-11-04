fn main() {
    let user1 = User {
        name: String::from("aaron"),
        age: 18,
        is_admin: false,
    };
    println!("user 1 name: {}", user1.name);

    let user2 = new_user(String::from("anderson"), user1.age);
    println!("user 2 name: {}", user2.name);
    println!("user 2 age: {}", user2.age);
    println!("user is admin: {}", user2.is_admin);

    let admin1 = new_admin(String::from("admin1"), 108);
    println!("admin 1 name: {}",admin1.name);

    let admin2 = User{
        name:String::from("admin2"),
        ..admin1
    };
    println!("admin 2 name: {}",admin2.name);
    println!("admin 2 age: {}",admin2.age);
    
}

struct User {
    name: String,
    age: i32,
    is_admin: bool,
}
fn new_user(name: String, age: i32) -> User {
    User {
        name: name,
        age: age,
        is_admin: false,
    }
}

fn new_admin(name: String, age: i32) -> User {
    User {
        name,
        age,
        is_admin: true,
    }
}
