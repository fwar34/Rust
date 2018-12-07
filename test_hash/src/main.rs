use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    
    let teams = vec![String::from("Blue"), String::from("Green")];
    let init_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    let key = String::from("Favorite color");
    let value = String::from("Red");
    let mut map = HashMap::new();
    map.insert(key, value);
    // key and value are invalid at this point, try using them and
    // see what compiler error you get!

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (k, v) in &scores {
        println!("{} : {}", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    println!("==========================");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        //or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
