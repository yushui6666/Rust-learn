fn main() {
    //所有权移动
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2，s1 不再有效
    println!("s2 = {}", s2);

    // 克隆
    let s3 = s2.clone(); // 深拷贝
    println!("s3 = {}", s3);
    println!("s2 = {}", s2); // 编译错误，s2 已被移动

    // 所有权与函数
    fn take_ownership(s: String) {
        println!("take_ownership got: {}", s);
    }
    take_ownership(s3); // s3 移动进入函数
    let  mut  s = String::from("hello");
    let  mut  s1 = s;
    s1.push_str(" world");
    println!("s1 = {}", s1);

}
