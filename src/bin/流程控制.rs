fn main() {
    // let number = 7;

    // // if 表达式
    // if number < 5 {
    //     println!("小于5");
    // } else {
    //     println!("大于等于5");
    // }

    // // if 作为表达式赋值
    // let result = if number % 2 == 0 { "偶数" } else { "奇数" };
    // println!("number 是 {}", result);

    // // loop 循环
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     if count == 3 { break; }
    // }
    // println!("loop 结束，count = {}", count);

    // // while 循环
    // let mut n = 1;
    // while n <= 4 {
    //     println!("while n = {}", n);
    //     n += 1;
    // }

    // // for 循环
    // for i in 0..=5 {
    //     println!("for i = {}", i);
    // }
    // let num = 4;
    // match num %2{
    //     0 => println!("偶数"),
    //     1 => println!("奇数"),
    //     _ => println!("其他"),
    // }
    let a =[1,2,3,4,5,6];
    for i in a.iter(){
        println!("{}",i);
    }
    for i in (1..=9).rev(){
        println!("{}",i);

    }
}
