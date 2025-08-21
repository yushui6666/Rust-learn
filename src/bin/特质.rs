// 定义 Trait
trait Summary {
    fn summarize(&self) -> String;
}

// 实现 Trait
struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}

fn main() {
    let article = Article { 
        headline: String::from("Rust is great!"), 
        content: String::from("Content") 
    };
    println!("Summary: {}", article.summarize());
    
}
