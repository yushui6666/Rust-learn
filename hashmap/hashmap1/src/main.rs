use std::collections::HashMap;
fn main(){
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); 
    // let vec = vec![("key1","value1"),("key2","value2")];
    // let map: HashMap<_,_> = vec.into_iter().collect();
    // 访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}",score);
    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let a = String::from("sdsd");
    let b = String::from("sdsdhjfkjs");
    let  mut l : HashMap<String,String> = HashMap::new();
    l.insert(a,b);
    //println!("{a},{b}");//所有权转移    
    //更新hashmap
    scores.insert(String::from("Blue"), 90);
    scores.entry(String::from("yellow")).or_insert(100);
    scores.entry(String::from("green")).or_insert(100);
    println!("{:?}",scores);
    // 统计单词出现次数
    let text: &'static str = "hello world wonderful world hello";

    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    //题目
    // let mut g = HashMap::new();
    // g.insert("k1",1);
    // let v1 = &g["k1"];//不可变引用
    // g.insert("k2",2);//可变引用
    // let v2 = &g["k2"];
    // println!("{},{}",v1,v2);
    





}