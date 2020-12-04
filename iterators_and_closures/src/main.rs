use std::thread;
use std::time::Duration;
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let _expensive_closure = |num: u32| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(3));
        // todo 提出类似这种东西 如何做好边界（什么防御性编程。。。）
        // num - 100
        num
    };
    // expensive_closure(2);


    fn _add_one_v1(x: u32) -> u32 {
        x + 1
    };
    let _add_one_v1 = |x: u32| x + 1;
    let _add_one_v2 = |x: u32| x + 1;
}

fn _some_expensive_calculation(intensity: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    println!("calculating....");
    intensity
}


// 这里的例子，对比下goroutine的执行
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|intensity| {
        thread::sleep(Duration::from_secs(5));
        println!("calculating....");
        intensity
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

//Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
