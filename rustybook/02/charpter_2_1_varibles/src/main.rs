fn main() {
    variables_1();
    variables_2();
    variables_3();
}

// 变量析构
fn variables_3() {
    let(a, mut b) :(bool, bool) = (true,  false);
    // a = true，不可变；b = false，可变
        println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn variables_2() {
    let _x = 5;
    let y = 10;
}

fn variables_1() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
