use std::error::Error;
use std::fs;
use std::env;

/// 运行程序的主要逻辑：读取文件并根据查询条件搜索
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容，遇到错误直接返回
    let contents = fs::read_to_string(&config.filename)?;

    // 根据大小写敏感性选择不同的搜索函数
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // 如果没有找到结果，提示用户
    if results.is_empty() {
        println!("没有找到匹配结果。");
    } else {
        // 否则打印所有匹配行
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

/// 配置结构体，用来保存命令行参数和搜索设置
pub struct Config {
    pub query: String,         // 用户输入的搜索关键字
    pub filename: String,      // 要搜索的文件名
    pub case_sensitive: bool,  // 是否大小写敏感
}

impl Config {
    /// 构建 Config：解析命令行参数
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("非法操作,请输入正确的参数".to_string());
        }

        // 第一个参数是程序名，所以跳过，从 args[1] 开始取
        let query = args[1].clone();
        let filename = args[2].clone();

        // 如果设置了环境变量 IGNORE_CASE → 大小写不敏感
        // 否则默认大小写敏感
        let case_sensitive = env::var("IGNORE_CASE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod test {
    use crate::{search, search_case_insensitive};

    /// 测试大小写敏感搜索
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        // 只匹配小写 "duct"
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// 测试大小写不敏感搜索
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        // 忽略大小写后可以匹配 "Rust:" 和 "Trust me."
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

/// 大小写敏感搜索
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// 大小写不敏感搜索
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 将 query 转成小写
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // 把每一行转成小写后比较
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
