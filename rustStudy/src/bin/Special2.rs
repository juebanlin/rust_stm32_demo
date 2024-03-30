/// impl<'a>Add <&'a i32>for i32.
/// impl<'a>Add < i32>for i32.

fn main() {
    test1();
    test2();
}

fn test1(){
    let list=vec![1,2,3];
    let mut x=0;
    for i in &list {
        x=i+1;//加法运算实现了解引用
    }
    println!("{}",x);
}

fn test2(){
    let list=vec![1,2,3];
    let mut x=0;
    for i in &list {
        x= *i;
    }
    let y=list.get(0).unwrap();
    println!("{}",x);
    println!("{}",y);
}