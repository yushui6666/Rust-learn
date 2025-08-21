fn main() {
    // 整数类型
    let a: i32 = 100;
    let b: u32 = 200;

    // 浮点类型
    let x: f32 = 3.14;
    let y: f64 = 2.71828;

    // 布尔类型
    let t: bool = true;

    // 字符类型
    let c: char = 'R';

    // 元组：可以存储不同类型的值
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    println!("元组第一个元素: {}", tup.0);

    // 数组：固定长度，元素类型相同
    let arr: [i32; 3] = [1, 2, 3];
    println!("数组第二个元素: {}", arr[1]);
}
