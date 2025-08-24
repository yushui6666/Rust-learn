// 定义结构体
// #[derive(Clone, Debug)]
// struct user {
//     username: String,
//     email: String,
//     active: bool,
//     sign_in_count: u64,
// }
// fn built_user(username: String,email: String) -> user{
//     user{
//         username,
//         email,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     // 创建结构体实例
//     let  user1 = user {
//         username: String::from("alice"),
//         email: String::from("alice@example.com"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let user2:user = user{
//         active: false,
//         ..user1.clone()

//     };

//     println!("Username: {}", user1.username);
//     println!("Email: {}", user1.email);
//     println!("Active: {}", user1.active);
//     println!("Sign-in count: {}", user1.sign_in_count);
    

// }
#[derive(Debug)]
struct Rec {
    w: u32,
    l: u32,
}

impl Rec {
    fn area(&self) -> u32 
    {
        self.w * self.l
    }
    fn daxiao(&self,other:&Rec) ->bool

    {
        self.area()>other.area()
    }
    fn zheng(size:u32) ->Rec
    {
        Rec
        {
            w:size,
            l:size,
        }
    }
}

fn main() {
    let r = Rec { w: 30, l: 40 };
    let r2 = Rec { w: 20, l: 30 };
    let r3 = Rec::zheng(20);
    println!("r是否大于r2 = {}", r.daxiao(&r2));
    println!("r3的面积 = {}", r3.area());
    println!("面积 = {}", r.area());
}
