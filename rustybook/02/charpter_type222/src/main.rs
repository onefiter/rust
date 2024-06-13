fn main() {
    charfn();
    unicodefn();
    boolfn();
}

// TODO 单元类型()

// bool类型
fn boolfn() {
    let _t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
}

// unicode
/**
 * Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，
 * 包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。
 * Unicode 值的范围从 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF。
 * 不过“字符”并不是 Unicode 中的一个概念，所以人在直觉上对“字符”的理解和 Rust 的字符概念并不一致。
 * 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
 */
fn unicodefn() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}

// 字符类型
fn charfn() {
    let c = 'z';
    println!("{}", c);

    let z = 'ℤ';
    println!("{}", z);
    let g = '国';
    println!("{}", g);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
}
