
/**
默认 Rust 中，对一个资源，同一时刻，有且只有一个所有权拥有者。
Rc 和 Arc 使用引用计数的方法，让程序在同一时刻，实现同一资源的多个所有权拥有者，多个拥有者共享资源。

Rust中提供了只读引用的类型有&、Rc、Arc等指针
Rust中提供了内部可变性的类型有Cell、RefCell、Mutex、RwLock以及Atomic*系列类型等。
**/

use std::rc::*;
use std::io::stdin;


fn main() {
    test3();
}

/// 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
/// 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
/// Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
/// Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。
fn test1(){
    let str="hello".to_string();
    let str2=Rc::new(String::from("world"));
    str.trim();
    str2.trim();//不影响包裹对象的方法调用形式


    let five=Rc::new(5);//存在堆上的数值
    let f1=five.clone();//增加一个拥有者
    let f2=five.clone();
}

/// Rc 是一个引用计数指针，而 Weak 是一个指针，但不增加引用计数，是 Rc 的 weak 版。
/// 可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
/// 可由 Rc<T> 调用 downgrade 方法而转换成 Weak<T>；
/// Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 值为 None；
/// 常用于解决循环引用的问题。
fn test2(){
    let five=Rc::new(5);
    let weak=Rc::downgrade(&five);
    let strong = weak.upgrade().expect("引用已被释放,无法升级");
}

/// Arc 是原子引用计数，是 Rc 的多线程版本
/// Arc 可跨线程传递，用于跨线程共享一个对象；
/// 用 Arc 包裹起来的类型对象，对可变性没有要求；
/// 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
/// Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
/// Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）
/// Arc 也有一个对应的 Weak 类型，从 std::sync::Weak 引入。意义与用法与 Rc Weak 基本一致
fn test3(){
    use std::sync::Arc;
    use std::thread;
    let list=(1..3i32).collect::<Vec<_>>();
    let arc_list=Arc::new(list);//对可变性没有要求
    //启动多个线程进行list的只读访问
    for _ in 0..2{
        let tmp_list=arc_list.clone();//增加一个引用
        //开启一个线程去使用
        thread::spawn(move||{
           let var=&tmp_list[..];//切片转数组
            println!("{:?}",var);
        });
    }
    let arc_list_2=arc_list.clone();//新增一个拥有者
    drop(arc_list);//手动删除引用
    //等线程跑完
    let mut end=String::new();
    stdin().read_line(&mut end).unwrap();
    println!("{:?}",arc_list_2);//arc_list_2还持有引用者,值有效
}