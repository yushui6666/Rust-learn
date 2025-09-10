
    // let r;
    // {
    //     let f = 9;
    //     r = &f;//f的生命周期
    // }
    // println!("{}", r);//r的生命周期
    // let r = 9;
    // let l = &r;
    // println!("{}", l);
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// struct lll<'a>{
//     first: &'a str,
// }
// fn main() {
    // let a = String::from("sd");
    // let c ;
    // let b = String::from("dsdd");//b的生命周期
    // c = longest(&a.as_str(), &b.as_str());
    // println!("The longest string is {}", c);//c的生命周期
    // let fj = String::from("sd jdkjf fksljdfjk fshjdlfjs fsddkjf");
    // let fir = fj.split('.').next().unwrap();
    // let d: lll<'_> = lll{
    //     first: fir,
    // };
    // println!("{}", d.first);
//     1. 关于生命周期，一切都与引用有关。
//     2. 把函数看作一个引用的黑箱，不要管内部，只看输入与输出。
//      ① 单进单出，没问题（原进原出）。
//      ② 多进单出，选短的（先烈优待）。
//      ③ 零进单出，绝对错（量入为出）。
//     3. 涉及到结构体：人 {  &钱  }
//      ① 正常情况：结构体实例持有的引用比它自身更长寿。（人挂了，钱还没花完）
//      ② 悲惨情况：结构体实例比它持有的引用更长寿（人还没挂，钱早没了）
//}
use std::fmt::Display;

fn longest<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("sd");
    let b = String::from("dsdd"); // b 的生命周期
    let c = longest(&a, &b, "测试一下"); // ann 参数需要传一个能 Display 的东西，比如字符串字面量
    println!("The longest string is {}", c); // c 的生命周期
}
