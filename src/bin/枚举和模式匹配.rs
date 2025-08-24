// 枚举类型
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::Write(String::from("hello"));

//     // match 模式匹配
//     match msg {
//         Message::Quit => println!("Quit"),
//         Message::Move { x, y } => println!("Move to {},{}", x, y),
//         Message::Write(text) => println!("Write: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Color: {},{},{}", r, g, b),
//     }
// }
// enum Message {
//     v4(u8,u8,u8,u8),
//     v6(String),
// }
// impl Message

// {
//     fn ma(&self) -> &str
//     {
//         match self {
//             Message::v4(_,_,_,_) => "v4",
//             Message::v6(_) => "v6",
//         }
//     }

// }

// fn main()
// {
//     let ip = Message::v4(192,168,1,1);
//     let ip2 = Message::v6(String::from("192.168.1.1"));
//     println!("{}",ip.ma());
//     println!("{}",ip2.ma());
// }
//option枚举
// fn main()
// {
//     let x: i8= 5;
//     let Y:Option<i8> = Some(5);
//     let sum = x+y;




// }
//match匹配
// fn main(){
//     let five = Some(5);
//     let six = add1(five);
//     let none = add1(None);
// }
// fn add1(x:Option<i32>) ->Option<i32>{
//     match x{

//         None => None,
//         Some(x) => Some(x+1),
//     }//match必须处理所有情况 

// }
//if let只关心一种情况

fn main(){
    let v = Some(0u8);
    match v{
        Some(v) => println!("{}",v),
        _=>println!("none"),
    }
    if let Some(x) = v{
        println!("{}",x);
    }
    else{
        println!("none");

    }

}