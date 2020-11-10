use std::collections::HashMap as hm;
fn main() {
    let mut map = hm::new();
    map.insert(1, 2);
    map.insert(2, 2);
    map.insert(3, 2);
    map.insert(4, 2);
    map.insert(5, 2);
    map.insert(6, 2);
    println!("map: {:?}", map)
}