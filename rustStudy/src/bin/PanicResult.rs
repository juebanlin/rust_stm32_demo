use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

///Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）
/// 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。
/// Rust 并没有异常，但是，有可恢复错误 Result<T, E> ，和不可恢复(遇到错误时停止程序执行)错误 panic!。
///
/// 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
/// 另一种选择是直接 终止（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。
/// 如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。
/// 例如，如果你想要在release模式中 panic 时直接终止：
/// [profile.release]
/// panic = 'abort'


///我们可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace
/// 为了获取带有这些信息的 backtrace，必须启用 debug 标识。当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用
fn main() {
    test5();
}

///直接的宏调用
fn test1(){
    //调用panic
    panic!("crash and burn");
}

///bug引起的别的库的panic
fn test2(){
    let v=vec![1,2,3];
    v[4];//尝试访问超越 vector 结尾的元素，这会造成 panic!
}


///Result 可恢复错误
fn test3(){
    let f=File::open("hello.txt");
    let file=match f {
        Ok(f1) => f1,
        Err(e) => match e.kind() {
            //如果是文件不存在
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f2) => f2,
                Err(e) =>  panic!("create file error:{:?}",e),
            },
            other_error => { panic!("other error:{:?}",e)},
        },
    };
}

///失败时,panic的简写处理
fn test4(){
    let f=File::open("hello.txt").unwrap();//使用默认的异常信息
    let f2=File::open("hello.txt").expect("failed to open");//使用指定的异常信息
}


///异常的传播
/// Result类型 和 ? 运算符
fn test5(){
    //手动匹配并返回异常
    fn readFromFile(path:& String)->Result<String,io::Error>{
        let mut f=match File::open(path) {
            Ok(f)=> f,
            Err(e)=>return Err(e),
        };
        let mut s=String::new();

        let result=match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        };
        return result;
    };

    /// ? 运算符可被用于返回值类型为 Result 的函数,如果使用？所在的函数不是此类型则会报错
    /// 只能在返回 Result 或者其它实现了 std::ops::Try 的类型的函数中使用 ? 运算符
    fn readFromFile2()->Result<String,io::Error>{
        //使用?运算符返回异常,如果结果是异常则向上抛
        let mut f=File::open("hello.txt")?;//结尾的 ? 将会把 Ok 中的值返回给变量 f。如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者。
        let mut s=String::new();
        f.read_to_string(&mut s)?;
        println!("s:{}",s);

        let mut s2=String::new();
        File::open("hello.txt")?.read_to_string(&mut s2)?;
        return Ok(s2);
    };
}




