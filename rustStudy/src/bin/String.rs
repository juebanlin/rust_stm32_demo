///String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型

fn main() {

    //新建空字符串
    let mut s=String::new();


    //使用to_string()方法从字面量创建String
    let data="hello world";

    let s=data.to_string();

    let s ="hello world".to_string();

    let s = String::from("hello world");

    test1();

    test2();

    test3();

    test4();

    test5();
}

///UTF-8支持
fn test1(){
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

///更新字符串
fn test2(){
    let mut s=String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("{}",s);
}

///所有权
fn test3(){
    let mut s1=String::from("hello");
    let s2=" world";
    s1.push_str(s2);
    println!("s1:{}",s1);
    println!("s2:{}",s2);
}

///使用+运算符和format宏拼接字符串
fn test4(){
    let s1=String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;//注意,s1的所有权被移动了,不能继续使用
    println!("s2:{},s3:{}",s2,s3);
    let s4=format!("{}-{}",s2,s3);
    println!("s4:{}",s4);
}

///索引字符串
fn test5(){
    let s1="hello";
    //let a1=&s1[0];
    //println!("a1:{}",a1);
    //Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
    let s=String::from("hello world");
    // let h=s[0];//不确定

    //如果你需要操作单独的 Unicode 标量值，最好的选择是使用 chars 方法。
    //遍历字符串
    for x in s.chars() {
        println!("char:{}",x);
    }
    for x in s.bytes() {
        println!("byte:{}",x);
    }
}


