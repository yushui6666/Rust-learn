use std::string;

fn main() {
    // // 不可变变量
    // let x = 5;
    // println!("不可变变量 x = {}", x);

    // // 可变变量
    // let mut y = 10;
    // println!("可变变量 y = {}", y);
    // y += 5;
    // println!("修改后的 y = {}", y);

    // // 常量
    // const MAX_POINTS: u32 = 100_000;
    // println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // // Shadowing（遮蔽）
    // let x = x + 1; // 使用同名变量覆盖之前的 x
    // println!("遮蔽后的 x = {}", x);
    // let num :u32= "45".parse().expect("Not a number!");
    // println!("num = {}",num);
    // let x = 2.0;
    // let y: f64 = 3.0;
    // let boole = false;
    // let char = 'a';
    // let string = "hello";
    // let tule = (1,-1,'f',4.4);
    // let (a,b,c,d) = tule;
    // println!("{}",tule.2);
    let month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a=1;
    println!("{}",month[a]);




}
 