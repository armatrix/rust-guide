use std::collections::HashMap;
fn main() {
    let mut ages: HashMap<&str, i32> = HashMap::new();
    ages.insert("aaron", 10);

    let mut scores = HashMap::new();
    let team1 = String::from("Blue");
    let team2 = String::from("Yellow");
    scores.insert(&team1, 10);
    scores.insert(&team2, 50);
    println!("scores: {:?}", scores);
    println!("team1: {:?}", team1);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];
    let v_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("v_scores: {:?}", v_scores);
    // 注意这里的返回值和之前的option是一样的，有和没有 v和none
    println!("v_scores[blue]: {:?}", v_scores.get(&String::from("Blue")));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
