fn main() {
    // Vector：可变长度数组
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("Vector: {:?}", v);

    // HashMap：键值对集合
    let mut map = std::collections::HashMap::new();
    map.insert("Alice", 25);
    map.insert("Bob", 30);
    println!("HashMap: {:?}", map);

    // HashSet：唯一元素集合
    let mut set = std::collections::HashSet::new();
    set.insert("apple");
    set.insert("banana");
    println!("HashSet: {:?}", set);
}
