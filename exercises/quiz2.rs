// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase=>{
                    output.push((&string).to_uppercase());
                }

                Command::Trim=>{
                    output.push((&string).trim().to_string());
                }
                
                Command::Append(value)=>{
                    let repeat_string = "bar".repeat(*value);  //
                    let tmp = string.to_string() + &repeat_string; //String + &str
                    output.push(tmp);

                }
                /*
                Rust中有两种主要的字符串类型：String 和 &str。
                String 类型是堆分配的可变字符串，而 &str 是静态分配的不可变字符串。
                在示例中，s1 是一个字符串切片，它指向静态分配的文本，例如字符串常量。
                当与 s2 进行合并时，可以使用 to_string() 方法将 s1 转换为 String 类型，
                这样两个字符串就可以使用 + 运算符进行合并了。

                let s1 = "Hello";
                let s2 = " World";
                let combined = s1.to_string() + s2;
                 */

            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
