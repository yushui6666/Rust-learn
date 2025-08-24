// 定义模块
mod greetings {
    pub fn hello() {
        println!("Hello from module!");//pub解除了模块的封装性

    }
}

fn main() {
    // 调用模块中的函数
    greetings::hello();
}
