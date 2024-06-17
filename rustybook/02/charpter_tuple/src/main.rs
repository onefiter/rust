fn main() {
    tupfn();
    tupfn_v2();
}

fn tupfn_v2() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("x.0:{},x.1:{},x.2:{}", five_hundred, six_point_four, one)
}

fn tupfn() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
