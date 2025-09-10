pub trait Person {
    fn name(&self) -> String {  //   String
        format!("(read more ~{})", self.address())
    }
    fn address(&self) -> &str {
        "skjdhfkshfklsfhns"
    }
}

pub struct Student {
    pub name: String,
    pub address: String,
}
impl Person for Student {
    fn name(&self) -> String {  //  自定义覆盖
        format!("{} (at {})", self.name, self.address)
    }
    fn address(&self) -> &str {
        &self.address
    }
}

pub struct Teacher {
    pub name: String,
    pub address: String,
}
impl Person for Teacher {
    fn address(&self) -> &str {
        &self.address
    }
    // Teacher 使用 trait 默认的 name()
}
