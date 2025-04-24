// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {  //&模式匹配
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

/*
use of partially moved value: `y`
部分移动（partial move）是Rust中一种常见的问题，它发生在将一个变量的部分值转移到另一个变量上，导致原始变量无法再继续使用。
 */