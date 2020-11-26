use std::fmt::Display;
fn main() {
    let name1 = "aaron";
    let name2 = "anderson";
    // let result;
    {
        let name4 = String::from("xyz");
        // result = longest(name1, name4.as_str())
    }
    println!("longest string: {}", longest(name1, name2));
    // println!("result: {}",result)

    let longest = longest_with_an_announcement(name1, name2, "some announcement");
    println!("longest string: {}", longest)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
