fn main() {
    intfn();
    floatfn();
    nanfn();
    number_calculator();
    number_calculatorv2();
    bitwise();
    rangefn();
}

/**
 * Rust 提供了一个非常简洁的方式，用来生成连续的数值，
 * 例如 1..5，生成从 1 到 4 的连续数字，不包含 5 ；1..=5，
 * 生成从 1 到 5 的连续数字，包含 5，它的用途很简单，常常用于循环中
 */
fn rangefn() {
    for i in 1..=5 {
        println!("{}", i);
    }

    /**
     * 序列只允许用于数字或字符类型，原因是：它们可以连续，
     * 同时编译器在编译期可以检查该序列是否为空，字符和数字值是
     * Rust 中仅有的可以用于判断是否为空的类型。
     */
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

// 位运算
fn bitwise() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn number_calculatorv2() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

fn number_calculator() {
    // 加法
    let sum = 5 + 10;
    println!("sum:{}", sum);

    // 减法
    let difference = 95.5 - 4.3;

    println!("difference:{}", difference);
    // 乘法
    let product = 4 * 30;
    println!("product:{}", product);
    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient:{}", quotient);
    // 求余
    let remainder = 43 % 5;
    println!("remainder:{}", remainder);
}

// 对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt() ，
// 会产生一个特殊的结果：Rust 的浮点数类型使用 NaN (not a number)来处理这些情况。
fn nanfn() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

fn floatfn() {
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    // 断言0.1 + 0.2与0.3相等
    // thread 'main' panicked at src/main.rs:10:5:
    // assertion failed: 0.1 + 0.2 == 0.3
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // assert!(0.1 + 0.2 == 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    //     abc (f32)
    //    0.1 + 0.2: 3e99999a
    //          0.3: 3e99999a

    // xyz (f64)
    //    0.1 + 0.2: 3fd3333333333334
    //          0.3: 3fd3333333333333

    // thread 'main' panicked at src/main.rs:29:5:
    // assertion failed: xyz.0 + xyz.1 == xyz.2

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
}

// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
fn intfn() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);

    println!("{}", b); // 19
}
