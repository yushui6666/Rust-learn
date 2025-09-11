use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!(" {}", err);
        process::exit(1);
    });
    println!("你要执行的操作为=》{}", config.query);
    println!("你要找的文件为=》{}", config.filename);
    if let Err(e) = run(config) {
        eprintln!("读取文件失败={}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    println!("文章如下\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("非法操作,请输入正确的参数".to_string());
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
