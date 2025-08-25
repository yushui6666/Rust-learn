fn main(){
    let mut s: String = String::new();
    let data: &'static str = "sjkgfhshbfskfdksfnks";
    let data_s: String = data.to_string();
    s.push('h');
    //s.insert(4, 'h');
    s.push_str(&data_s);
    println!("{}",s);

}