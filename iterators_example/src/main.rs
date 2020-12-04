fn main() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.iter();
    v1_iter.next(); // 9
    v1_iter.next(); // 7
    v1_iter.next(); // 4
    v1_iter.next(); // 0

    // 对迭代器里面的元素做操作
    let tally: i32 = v1_iter.sum();
    println!("tally: {}", tally);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum: {}", sum);

    let iter = Counter::new();
        // .zip(Counter::new().skip(2));
        // .map(|(a, b)| a * b);
        // .filter(|x| x % 3 == 0);

    for ele in iter {
        println!("data: {:?}", ele);
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
