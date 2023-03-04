fn main() {
    // let 定义不可变变量
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    // 编译报错, 需用let mux x = 5
    x = 6;
    println!("The value of x is: {}", x);
}
