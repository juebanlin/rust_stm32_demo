///方式1：声明同级的其它文件的模块utils
mod utils;

///方式2可以是同级的一个mylib.rs也可以是mylib目录下面的mod.rs
mod mylib;

mod proto_msg;

/**
优先查找xxx.rs 文件
main.rs、lib.rs、mod.rs中的mod xxx; 默认优先查找同级目录下的 xxx.rs 文件；
其他文件yyy.rs中的mod xxx;默认优先查找同级目录的yyy目录下的 xxx.rs 文件；
如果 xxx.rs 不存在，则查找 xxx/proto_msg 文件，即 xxx 目录下的 proto_msg 文件。
**/

//mod类似java的package ,同包可以方法
//一个rs的内容是同一个包

/**
Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
pub可以修饰mod,函数,结构体,枚举,属性
**/

fn build_hello_str3() ->String{
    return String::from("hello world 3");
}

mod tool{
    pub mod print{
        pub fn do_say_hello1(){
            println!("hello world 1");
        }

        pub fn do_say_hello2(){
            println!("hello world 2");
        }

        ///子模块调用顶层模块下的函数
        pub fn do_say_hello3(){
            //调用上级模块的方法
            super::toolinfo();
            //调用上级的上级模块的方法
            let str=super::super::build_hello_str3();
            println!("{}",str);
        }
    }

    pub fn toolinfo(){
        println!("toolinfo");
    }

    pub fn toolinfo3(){
        println!("toolinfo3");
    }
}

use crate::tool::toolinfo;

use crate::tool::toolinfo as toolinfoV2;

use crate::tool as _tool;

///使用 pub use重导出,将本模块才可用的函数对外再次公开
use crate::tool::toolinfo3 as toolinfoV3;
use proc_macro::TokenStream;

fn say_hello_world(){
    //因为tool和当前函数定义于同一模块中,所以不需要添加pub标记
    //tool的print需要pub来向上公开,类似java 中,A,B同包,A调用B类中的public修饰的内部类中的public方法
    //绝对路径
    crate::tool::print::do_say_hello1();
    //相对路径
    tool::print::do_say_hello2();
    tool::print::do_say_hello3();

    ///使用use关键词引入模块函数,降低调用路径
    toolinfo();
    //使用as关键字建立别名后调用
    //方式1-不推荐：使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化
    toolinfoV2();
    //方式2
    _tool::toolinfo();
}


// pub extern fn dll_lib_test(a:i32,b:i32)->i32{
//     return a+b;
// }

#[proc_macro]
pub fn make_hello(_item:TokenStream)->TokenStream{
    r#"fn hello()->() {println!("hello")}"#.parse().unwrap()
}