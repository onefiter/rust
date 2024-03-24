fn main() {
    v0();
    v1();
    v2();
    v3();
    v4();
    v5();
    v6();
    v7();
}

fn v7() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    print!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert!(a, "{}", b);
}

fn v6() {
    // 打开注释，会编译报错
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    print!("The value of v6 guess is: {}\n", guess)
}
fn v5() {
    // let mut spaces = "  ";
    // let spaces = spaces.len();
    // 隐藏机制生效演示
    let spaces = "  ";
    let spaces = spaces.len();
    print!("The value of spaces is: {}\n", spaces);
}

fn v4() {
    // 隐藏机制
    // 隐藏机制与mut的另一个区别在于：由于重复使用let关键字会创建出新的变量，所以我们可以在复用变量名称的同时改变它的类型
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    print!("The value of v4  x  is {}\n", x)
}

fn v3() {
    // 常量可读性
    const MAX_POINTS: u32 = 10_000;

    print!("MAX_POINTS of v3 is: {}\n", MAX_POINTS);
}

fn v2() {
    // 使用下划线开头忽略未使用的变量
    let _x = 5;

    // 打开注释，编译会警告
    // let y = 6;
}

fn v1() {
    // let 定义不可变变量
    // let x = 5;
    let mut x = 5;
    println!("The value of v1 x is: {}", x);
    // 编译报错, 需用let mux x = 5
    x = 6;
    println!("The value of x is: {}", x);
}

fn v0() {
    let x = 5;
    print!("The value of v0 x is {}\n", x);
    // 打开注释编译错误
    // x = 6;

    print!("The value of v0 x is {}\n", x);
}
