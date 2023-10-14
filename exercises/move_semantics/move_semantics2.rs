// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {

    let mut  vec0 = Vec::new();

    vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0);


    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);//调试表示打印

    //vec0.push(88); // 这里会出错，因为 fill_vec不返回值了

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);

}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32>  {
    let mut vec = vec.to_vec();

    vec.push(88);
    vec


}

// 这段 Rust 代码的 bug 是在调用 fill_vec 函数时传递了 vec0 的所有权，导致在之后的代码中无法使用 vec0。解决这个问题的方法有以下三种：
// 1. 创建一个独立的副本 vec0 的数据 ,let mut vec1 = fill_vec(vec0.clone());
// output : 
//vec0 has length 0 content `[]`
//vec1 has length 4 content `[22, 44, 66, 88]`

// 2. 改变 fill_vec 函数，使其借用参数而不获取所有权，并在函数内部复制数据以返回一个拥有的 Vec<i32>。
// 14 let mut vec1 = fill_vec(&vec0);
// 24 fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {     
// 25   let mut vec = vec.to_vec();
// output:
// vec0 has length 0 content `[]`
// vec1 has length 4 content `[22, 44, 66, 88]`


// 3. 在 fill_vec 函数中使用可变引用 &mut Vec<i32> 借用参数，并直接修改它，无需返回任何内容。这将需要将 vec1 声明为可变引用。
// 目前的代码就是第三个方法
// 第三个方法不能完全解决，需要明白的是 借用& 和可变借用 &mut 区别，借用只是把值借给他，不会影响原来的值， 但是可变借用类似 c++ ref