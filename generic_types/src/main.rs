fn main() {
    let arr = vec![34, 50, 25, 100, 65];
    let largest = largest(&arr);
    println!("largest: {:?}", largest);

    let p1 = Person {
        id: "aaron",
        character: "dispensable people",
    };
    let admin = Person {
        id: 001,
        character: "system admin",
    };
    p1.add_character(admin);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if largest < item {
            largest = item;
        }
    }
    largest
}

// 这里模拟一个个体，假定当id为字符串时表示其身份标识，为数字时表示某一人物角色，在系统中，通过某种条件可以出发add_character()方法
struct Person<T, U> {
    id: T,
    character: U,
}

impl<T, U> Person<T, U> {
    // 这里要指定范型的类型
    fn add_character<V, W>(self, other: Person<V, W>) -> Person<T, W> {
        Person {
            id: self.id,
            character: other.character,
        }
    }
}
