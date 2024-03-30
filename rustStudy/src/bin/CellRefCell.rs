
/**
Rust中提供了只读引用的类型有&、Rc、Arc等指针，它们可以提供alias。
Rust中提供了内部可变性的类型有Cell、RefCell、Mutex、RwLock以及Atomic*系列类型等。
这两类类型经常需要配合使用。
如果你需要把一个类型T封装到内部可变性类型中去，要怎样选择Cell和RefCell呢？
原则就是，如果你只需要整体性地存入、取出T，那么就选Cell。如果你需要有个可读写指针指向这个T修改它，那么就选RefCell。

“取引用”操作符，如&、&mut，是不允许重载的。
因此，“取引用”和“解引用”并非对称互补关系。*&T的类型一定是T，而&*T的类型未必就是T。
*/
fn main() {
    test1();
    test2();
}

///Cell<T> 只能用于 T 实现了 Copy 的情况；
fn test1(){
    use std::cell::Cell;
    let c=Cell::new(5);
    c.set(6);
    println!("{}",c.get());
}

///在不确定一个对象是否实现了 Copy 时，直接选 RefCell；
/// 如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
/// RefCell 只能用于线程内部，不能跨线程；
/// RefCell 常常与 Rc 配合使用（都是单线程内部使用）；
fn test2(){
    use std::cell::RefCell;
    let c=RefCell::new(vec![1,2,3]);

    // //error1 同时只能有一个可变借用
    // let y1=c.borrow_mut();
    // let y2=c.borrow_mut();

    // //error2 同时只能有一个可变借用
    // let y1=c.borrow_mut();
    // let x=c.borrow();

    // //error3 同时只能有一个可变借用
    // let x=c.borrow();
    // let y1=c.borrow_mut();

    //实时借用
    c.borrow_mut().push(4);
    c.borrow_mut().push(5);

    //多个不可变借用
    let x=c.borrow();
    let y=c.borrow();

    println!("x:{:?}",x);
}


