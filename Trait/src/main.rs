use Trait::Person;
fn main(){
    let s = Trait::Student{
        name: String::from("s"),
        address: String::from("s"),
    };
    let t = Trait::Teacher{ 
        name: String::from("t"),
        address: String::from("t"),
    };
    println!("{}",s.name());
    println!("{}",t.name());
    println!("{}",s.address());
    println!("{}",t.address());
}
// use std::fmt::Display;
// fn displayable<T: Display>(t: T) -> impl Display {
//     t
// }
// fn main(){
//     let s = String::from("djskhsk");
//     let mut s2 = displayable(s);
//     s2.push_str("FUCK");
//     println!("{}",s2);
// }
    
