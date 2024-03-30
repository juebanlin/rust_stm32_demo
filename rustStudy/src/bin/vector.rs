#![feature(slice_ptr_len)]

use std::io::Write;
use std::ptr::slice_from_raw_parts;

fn main() {
    let mut vec:Vec<i32>=Vec::new();

    let array= [[1;2];2];//[[1,1],[1,1]]

    //添加元素
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //遍历集合
    for x in &vec {
        println!("i:{}",x);
    }

    //使用宏创建集合
    vec=vec![4,5,6];
    for x in &vec {
        println!("i:{}",x);
    }
    //通过迭代器生成
    vec=(1..5).collect();

    println!("{}",&vec[0]);
    println!("{}",&vec[1]);
    println!("{}",vec.get(2).unwrap_or(&-1));

    //当取的下标超过的时候,使用get会返回Null,因为get拿到的时option类型,更安全
    // println!("{}",&vec[100]);


    // let x=&vec[0];
    // vec.push(4);
    // println!("x:{}",x);// error 租借一个元素后立马修改集合会报错

    ///解引用运算符
    let mut list=vec![1,2,3];
    for x in &mut list {
        //使用解引用运算符（*）获取 x 中的值。
        *x=*x*10;
    }
    for x in &list {
        println!("x:{}",x);
    }

    //list添加复杂类型
    enum Number{
        Int(i32),
        Float(f64),
        Text(String)
    }
    let numbers=vec![Number::Int(1),Number::Float(1.0),Number::Text(String::from("hello"))];
    test2();
    test3();
    test4();
}

fn test2(){

    let s1=1;
    let s2=[1,2];
    let h1=(1..3).collect::<Vec<i32>>();
    let h2=(1..3).collect::<Vec<i32>>();

    let addr1=&s1 as *const i32 as usize;
    let addr2=&s2[0] as *const i32 as usize;


    let addr3=&(*Box::new(1i32)) as *const i32 as usize;
    let addr4=&h1[0] as *const i32 as usize;
    let addr5=&h2[0] as *const i32 as usize;

    println!("stack :{:X},{:X}",addr1,addr2);
    println!("{:X},{:X},{:X}",addr3,addr4,addr5);

    let mut v=vec![1];
    let mut v2:Box<Write>=Box::new(v);
    v2.write(b"1");
}

 fn test3(){

    let a=[1,2,3];
     let len =a.len();
     //数组引用转换为裸指针
    let x=&a as *const i32;
     //切片裸指针
    let y=slice_from_raw_parts(x,len as usize);
    let z=unsafe { &*y };//裸指针转引用
     println!("{:?}",z);
}

fn test4(){

    let a=[[1,2],[2,3]];
    let len =a.len();
    //数组引用转换为裸指针
    let x=&a as *const [* const i32];
    //切片裸指针
    let y= unsafe { std::ptr::read(x) };
    for v in &y{
        let z=unsafe { *v };
        let z2=slice_from_raw_parts(z,2);
        let z3=unsafe { &*z2 };
        println!("{:?}",z3);
    }
}


