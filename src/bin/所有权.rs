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
    //引用与借用
    // 引用
    // 引用是一种指针，它指向内存中的某个值
    // 引用不拥有它指向的值
    // 引用是不可变的
    // 引用的作用域从声明的地方开始一直持续到最后一次使用

    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    println!("s = {}", s);
    //悬垂引用
    // 悬垂引用
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // 编译错误，s 是一个局部变量，函数执行完毕后，s 会被销毁，引用 s 会悬垂

    // }
    //切片
    
}
