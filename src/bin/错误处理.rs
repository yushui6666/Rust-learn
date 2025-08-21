use std::fs::File;
use std::io::{self, Read};

// 返回 Result 类型的函数
fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("test.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file() {
        Ok(text) => println!("File content: {}", text),
        Err(e) => println!("Error: {}", e),
    }
}
