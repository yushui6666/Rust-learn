use std::error::Error;
use std::io;
use std::num::ParseIntError;
use std::fs::File; // 注意：File 需要引入
use std::io::Read; // 如果要读取文件
// // 返回 Result 类型的函数
// fn read_file() -> Result<String, io::Error> {
//     let mut f = File::open("test.txt")?;
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn main() {
//     match read_file() {
//         Ok(text) => println!("File content: {}", text),
//         Err(e) => println!("Error: {}", e),
//     }
// }
// fn main() {
//     // let gree = File::open("text.txt");
//     // let greeting = match gree {
//     //     Ok(file) => file,
//     //     Err(e) => match e.kind() {
//     //         ErrorKind::NotFound => match File::create("text.txt") {
//     //             Ok(file) => file,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         },
//     //         _ => panic!("Problem opening the file: {:?}", e),
//     //     },
//     // };

//     // println!("File ready: {:?}", greeting);
//     let  dd = File::open("text.txt").unwrap();
//     let ff = File::open("text.txt").expect("djkfhsjdkf");
// }
#[derive(Debug)]
pub enum MyError {
  Io(io::Error),
  ParseInt(ParseIntError),
  Other(String),
}
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseInt(err)
    }
}
fn Res() -> Result<String, MyError> {
    let mut name = String::new();
    let f =  File::open("text.txt")?.read_to_string(&mut name)?;
    let num = "56".parse::<i32>()?;
    Ok(name)
}
fn main(){
    let res = Res();
    match res {
        Ok(name) => println!("{}", name),
        Err(e) => println!("{:?}", e),
    }
}