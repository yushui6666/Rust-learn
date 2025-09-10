#[derive(Debug)]
struct fang {
    width: u32,
    height: u32,
}
impl fang {
    fn bigger(&self,other:&fang)->bool {
        self.width > other.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = fang{
            width: 10,
            height: 20,
        };
        let b = fang{
            width: 100,
            height: 200,
        };
        assert_eq!(a.bigger(&b),false);
    }
    #[test]
    fn test2() {
        let a = fang{
            width: 10,
            height: 20,
        };
        let b = fang{
            width: 100,
            height: 200,
        };
        assert_ne!(a.bigger(&b),false);
    }

}
