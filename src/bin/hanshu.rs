// 函数定义
fn add(a: i32, b: i32) -> i32 {
    a + b  // 函数最后一个表达式是返回值，不需要 `return`
}

fn main() {
    let sum = add(5, 10);
    println!("5 + 10 = {}", sum);

    // 无返回值函数
    fn print_hello() {
        println!("Hello, Rust!");
    }
    print_hello();
}
