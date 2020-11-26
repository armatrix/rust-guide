fn main() {
    let tweet = Tweet {
        username: String::from("aaron"),
        content: String::from("to be, or not to be"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("article headline"),
        location: String::from("location"),
        author: String::from("anderson"),
        content: String::from("content"),
    };

    println!("article: {}", article.summarize());
    println!("article author: {}", article.summarize_author());
    println!("tweet: {}", tweet.summarize());
    println!("tweet author: {}", tweet.summarize_author());

    notify(tweet);
    notify(article);

    println!("{}", 3.to_string())
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from {}...", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub trait another_trait {}
// trait 作为参数
pub fn notify(item: impl Summary) {
    println!("from: {:?}", &item.summarize())
}

// 语法糖
pub fn notify_v1<T: Summary>(item: T) {
    println!("from: {:?}", &item.summarize())
}

// 多个tarit作为参数，同样的
fn sunscribe<T: Summary>(channel1: impl Summary, channel2: impl Summary) {}
fn sunscribe_v1<T: Summary>(channel1: T, channel2: T) {}

// 多个trait
pub fn foo(item: impl Summary + another_trait) {}
pub fn bar<T: Summary + another_trait>(item: T) {}

// 对多个trait的较为清晰的写法
fn foo1<T, U>(t: T, u: U) -> i32
where
    T: Clone + Summary,
    U: Summary + another_trait,
{
    5
}

// 在返回中实现trait的类型 闭包和迭代器场景
fn return_summary() -> impl Summary {
    Tweet {
        username: String::from("aaron"),
        content: String::from("some content"),
        reply: false,
        retweet: false,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("read more from {}...", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@ {}...", self.username)
    }
}

// 当我们使用范型作为参数的时候，要注意我们的逻辑中对可能存在的范型实现造成的侵入，如这里我们将值进行传递，但可能存在一些所有权已经丢失的情况
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
