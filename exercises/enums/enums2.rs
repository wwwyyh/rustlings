// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
struct Stu1{
    x:i32,
    y:i32,
}
#[derive(Debug)]
struct Stu2(
    i32,i32,i32
);

#[derive(Debug)]
enum Message {
    Move{x:i32, y:i32},
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [ //注意messages是一个数据
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
