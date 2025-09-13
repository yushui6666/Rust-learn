use std::env;
use std::process;
use first_made_Mini_Grep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);//stderr

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!(" {}", err);
        process::exit(1);
    });
    println!("你要执行的操作为=》{}", config.query);
    println!("你要找的文件为=》{}", config.filename);
    if let Err(e) = first_made_Mini_Grep::run(config) {
        eprintln!("读取文件失败={}", e);
        process::exit(1);
    }
}

