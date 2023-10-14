// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); //"blue" 是&
    string("red".to_string());
    string(String::from("hi"));
    /*
    "rust is fun!".to_owned() 是一个 Rust 编程语言中的表达式。它的作用是将字符串 "rust is fun!" 转换为一个 String 类型的对象。
    在 Rust 中，字符串字面量默认是一个 &str 类型，它是一个指向存储字符串的内存的切片。当使用 to_owned() 方法时，它会创建一个新的 String 对象，将字符串的内容复制到新对象中，并返回该对象。
    因此，"rust is fun!".to_owned() 的含义是创建一个包含相同内容的 String 对象，该对象是字符串 "rust is fun!" 的副本。在 Rust 中，这种将字符串的切片复制到新的 String 对象的方式常用于创建、传递和处理字符串对象。
     */
    string("rust is fun!".to_owned());

    /*
    "nice weather".into() 是一个 Rust 编程语言中的表达式。它的作用与 "rust is fun!".to_owned() 类似，都是将字符串字面量转换为一个 String 类型的对象。
    在 Rust 中，通过调用 .into() 方法可以进行类型转换，根据目标类型的实现，该方法会将原始类型进行适当的转换。对于字符串字面量，.into() 方法会将其转换为 String 类型的对象。
     */
    string("nice weather".into());
    //format! 宏可以用于创建一个新的字符串
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //注意to_lowercase得到的是String
}
