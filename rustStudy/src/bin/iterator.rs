use std::sync;

fn main(){
    let v:Vec<_> =(1..5).collect();//显式地标明v的类型:
    let v2=(1..5).collect::<Vec<_>>();//显式地指定collect调用时的类型

    //范围迭代器
    let it=(1..10);
    for i in it{
        println!("{}",i);
    }

    for i in 1..5{
        println!("{}",i);
    }

    //无限迭代器
    let mut seq=(1..).into_iter();
    println!("seq:{}",seq.next().unwrap());//1
    println!("seq:{}",seq.next().unwrap());//2

    //mapreduce fold的输出结果的类型，最终是和base的类型是一致的
    let v1:Vec<i32> =(1..5).collect();//显式地标明v的类型:
    let v2:Vec<i32> =(1..5).collect();//显式地标明v的类型:
    let a=v1.into_iter().fold(1i32,|mux,x| mux * x);//1i32为mut起始值
    let b=v2.into_iter().map(|x|x*10).collect::<Vec<i32>>();
    println!("a:{}",a);
    println!("b:{}",b.get(0).unwrap());

    test1();
    test2();
}

fn test1(){

    let mut a=10;
    let b=loop {
        a=a-1;
        if a<5 {
            break a;
        }
    };
    println!("{}",b);

    let a:(i32,u32,f32)=(1,2,3.0);
    let (x,y,z)=a;
    println!("x:{},y:{},z:{}",x,y,z);


    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);//不允许比较数字的引用与数字，因为它们是不同的类型。必须使用解引用运算符追踪引用所指向的值。
}

fn test2(){
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    let s1=b"hello \n";//字节字符串
    let s2=br"hello \n";//字节字符串
    println!(r##"hello"##);//打印"hello"
}

