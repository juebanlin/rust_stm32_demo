use proc_macro::TokenStream;
use inline_python::pyo3::pyclass::PyClassAlloc;
use std::fmt::Debug;

///有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。
/// 如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。
/// 通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
fn main() {
    // rustStudy::say_hello_world();

    test_proc_macro();

    #[derive(Debug)]
    struct Point{
        x:i32
    }
    let p=Point::new;
    p.fmt();
}



/**
注意: 使用 Cargo 时，定义过程宏的 crate 的配置文件里要使用 proc-macro键做如下设置：
[lib]
proc-macro = true
 */


#[proc_macro]
fn make_hello(_item:TokenStream)->TokenStream{
    r#"fn hello()->() {println!("hello")}"#.parse().unwrap()
}

fn test_proc_macro() {
    rustStudy::make_hello!();
    hello();
}