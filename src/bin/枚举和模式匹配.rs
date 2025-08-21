// 枚举类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Write(String::from("hello"));

    // match 模式匹配
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {},{}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {},{},{}", r, g, b),
    }
}
