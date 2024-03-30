use std::fmt::{Result, Error};
use std::io::Result as IoResult;

//使用 extern 引入包之后，可以用 use 将模块引入到当前作用域。
// extern crate my_library;
// use my_library::english::greetings::hello;
// use my_library::english::farewells::goodbye;

///当使用不同包下相同名的类型时,使用as创建别名

fn main() {

}

fn fn1()->Result{
    return Ok(());
}

fn fn2()->IoResult<()>{
    return Ok(());
}

///嵌套路径消除大量的use行
// use std::cmp::Ordering;
// use std::cmp::io;
//优化后
use std::{cmp::Ordering,io};

// use std::clone;
// use std::clone::Clone;
//优化后
use std::clone::{self,Clone};

///glob运算符引入所有的公有定义
use std::collections::*;
use std::ptr::null;