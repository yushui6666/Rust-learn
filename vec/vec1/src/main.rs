enum biaoge {
        int(i32),
        flout(f32),
        str(String),

    }
fn main() {
    // Vector：可变长度数组
    //let v:Vec<i32> = Vec::new();
    // let mut v= vec![1,2,3];
    // v.push(4); 
    // let third: &i32 = &v[2];
    // println!("{}", third);
    // match v.get(9) {
    //     Some(third) => println!("The third element is: {}", third),
    //     None => println!("There is no third element."),
    // }
    
    let row =  vec![
        biaoge::int(1),
        biaoge::flout(2.0),
        biaoge::str(String::from("hello"))
        ];
}



