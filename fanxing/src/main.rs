fn lar<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = lar(&number_list);
    println!("The largest number is: {}", result);
    let words = vec!["d","s","f","a"];
    let result2 = lar(&words);
    println!("The largest words is :{} ",result2);
}
