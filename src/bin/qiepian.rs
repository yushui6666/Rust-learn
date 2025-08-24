
// fn main(){

// }
// fn first_word(s: &String) ->usize{
//     let words = s.as_bytes();
//     for(i,&item) in Bytes::iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }
fn main() {
    let s = String::from("hello world");
    let idx = first_word(&s);
    //s.clear();
    let  s=idx.len();
    println!("{}",idx);
    println!("First word ends at index: {}", s);
}

fn first_word(s: &str) -> &str 
{
    let words = s.as_bytes();

    for (i, &item) in words.iter().enumerate() 
    {
        if item == b'r' 
        {
            return &s[0..i];
        }
    }
    &s[..]
}
// fn main()
// {
//     let s = String::from("jshfskahgfalbfiWE4FLSJKfssdfhks");
//     let a = &s[0..5];
//     let b = &s[6..];
//     println!("{} {}",a,b);
// }
