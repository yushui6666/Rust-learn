// 定义模块
mod greetings {
    pub fn hello() {
        println!("Hello from module!");
    }
}

fn main() {
    // 调用模块中的函数
    greetings::hello();
}
