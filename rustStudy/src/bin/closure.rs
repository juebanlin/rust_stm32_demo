#![feature(core_intrinsics)]

use std::rc::Rc;
fn main() {
    test0();
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
}

fn test0(){
    {//Fn 不可变借用,不能改变外部环境变量,可多次调用
        let s="Fn".to_string();
        let f=||{println!("{}",s)};
        f();
        println!("{}",s);
    }
    {//FnMut 可变借用,能改变外部环境变量,可多次调用
        let mut s="FnMut".to_string();
        let mut f=||{
            s.push('x');
            println!("{}",s)
        };
        f();
        println!("{}",s);
    }
    {//FnOnce 转移所有权,不能改变外部环境变量,只能1次调用
        let s="FnOnce".to_string();
        let f=move ||{//移动s到闭包
            println!("{}",s);
            s
        };
        let s2=f();//移动出来
        println!("{}",s2);
    }
}

fn test1(){
    println!("###test1");
    let addfun1=|x:i32| x+1;//隐藏返回值
    let addfun3=|x:i32| ->i32 {x+1};
    let a=addfun1(1)+addfun3(2);
    println!("{}",a);
}

///包含局部变量
fn test2(){
    println!("###test2");
    let a=10;
    let addfun =|x:i32| x+a;
    let b=&a;//不可变借用
    let v=addfun(20);
    println!("{},{}",a,b);
    println!("{}",v);
}

///包含局部变量
/// 同一作用域下只能有一个可变借用(&mut T)
fn test3(){
    println!("###test3");
    let mut a=10;
    let addfun = |x:i32| x+a;
    //let b=&mut a;// error a已经被借走了,在闭包调用完之前,不能被借,在可变借用释放前不可访问源变量
    let v=addfun(20);
    let c=&mut a;//后面没有闭包调用,a可以借
    println!("{}",c);
    println!("{}",v);
}

///同一作用域，特定数据可有0个或多个不可变借用（&T），但不能有任何可变借用
fn test4(){
    println!("###test4");
    let mut a=10;
    let addfun = |x:i32| x+&a;
    let b=&a;// error a已经被借走了,在闭包调用完之前,不能被借
    let v=addfun(20);
    let c=&a;//后面没有闭包调用,a可以借
    println!("{}",b);
    println!("{}",c);
    println!("{}",v);
}

///闭包是trait实现的,闭包作为函数参数
fn test5(){
    println!("####test5");
    //声明一个闭包
    let fun=|x:i32|->i32{x+1};
    //一个接受闭包的函数
    fn invoke(fun:&Fn(i32)->i32)->i32{
        fun(1)
    }
    let result=invoke(&fun);
    println!("result:{}",result);
}

///泛型闭包参数
fn test6(){
    println!("####test6");
    //声明一个闭包
    let fun=|x:i32|->i32{x+1};
    //一个接受闭包的函数
    fn invoke1< T: Fn(i32)->i32 >(fun:&T)->i32{
        fun(1)
    }
    fn invoke2<T>(fun:&T) ->i32 where T:Fn(i32)->i32{
        fun(2)
    }
    let result1=invoke1(&fun);
    let result2=invoke2(&fun);
    let result3=invoke2(&|a| a+1);
    println!("result1:{}",result1);
    println!("result2:{}",result2);
    println!("result3:{}",result3);
}

///函数指针和闭包,一个函数指针有点像一个没有环境的闭包。
fn test7(){
    println!("####test7");

    //一个接受闭包的函数
    fn invoke<T>(fun:&T) ->i32 where T:Fn(i32)->i32{
        fun(1)
    }
    //声明一个闭包
    let cfun=|x:i32|->i32{x+1};
    let result1=invoke(&cfun);
    println!("result1:{}",result1);

    //情况二 Fn类型兼容fn
    fn add(i:i32)->i32{
        i+2
    }
    let fun=add;
    let result2=invoke(&fun);
    println!("result2:{}",result2);
}

///闭包作为返回值
fn test8(){
    println!("####test8");
    type FnType=Fn(i32)->i32;
    fn buildTask() ->&'static (Fn(i32)->i32){
        &|x|{x+1}
    }
    //通过Box装箱Fn来返回一个 trait 对象
    fn buildTask2() -> Box<FnType>{
        Box::new(|x|x+2)
    }
    fn buildTask3() -> Box<FnType>{
        let num=3;
        Box::new(move |x|x+num)//move捕获环境变量,num是i32基础类型,move实际是copy了值
    }
    fn buildTask4() -> Rc<FnType>{
        let num=3;
        Rc::new(move |x|x+num)//move捕获环境变量,num是i32基础类型,move实际是copy了值
    }
    let cfun=buildTask();
    let cfun2=buildTask2();
    let cfun3=buildTask3();
    let cfun4=buildTask4();
    let result=cfun(10);
    let result2=cfun2(20);
    let result3=cfun3(30);
    let result4=cfun4(40);
    println!("result1:{}",result);
    println!("result2:{}",result2);
    println!("result3:{}",result3);
    println!("result4:{}",result4);

}

fn test9(){
    let a=1;
    let b="".to_string();

    let fn1=||{};
    let fn2=|x:i32|{x+a};
    let fn3=move |x:i32|{x+a};
    let fn4=move ||{a};

    let fn5=||{b.to_lowercase()};
}