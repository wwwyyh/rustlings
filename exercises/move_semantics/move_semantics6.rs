// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let a = get_char(&data);
    println!("{}", a);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) ->  char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) {
    let data = data.to_uppercase();

    println!("{}", data);
}

// 第18行不需要解引用
// type `char` cannot be dereferenced
// 18 |     *data.chars().last().unwrap()

// 但是，下面这样写是可以的
// (*data).chars().last().unwrap()