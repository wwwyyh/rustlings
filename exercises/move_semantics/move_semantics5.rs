// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;

    let z = &mut x;
    
    *z += 1000;
    assert_eq!(x, 1200);
}






// fn main() {
//     let mut x = 100;
//     let y = &mut x;
//     let z = &mut x;
//     *y += 100;
//     *z += 1000;
//     assert_eq!(x, 1200);
// }

// error[E0499]: cannot borrow `x` as mutable more than once at a time
// --> exercises/move_semantics/move_semantics5.rs:11:13
// |
// 10 |     let y = &mut x;
// |             ------ first mutable borrow occurs here
// 11 |     let z = &mut x;
// |             ^^^^^^ second mutable borrow occurs here
// 12 |     *y += 100;
// |     --------- first borrow later used here

// error: aborting due to previous error
