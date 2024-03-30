use std::fmt::Display;

/// 生命周期和引用有效性
/// 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
/// 一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。
///
/// 生命周期省略规则：
/// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。
/// 第一条规则是每一个是引用的参数都有它自己的生命周期参数。
///  换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
///  有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
/// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
/// 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
///  说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章),
///  那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
/// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
fn main() {
    test1();
    test2();
    test3();
    test3_error();
    test4_error();
}

///悬垂引用,使用离开作用域的值的引用
fn test1(){
    let x;
    let z;
    {
        let a=5;
        let b=6;
        x=&a;
        z=b;//所有权移动到z
    }
    // println!("x:{}",x);//error 尝试使用离开作用域的值的引用
    println!("z:{}",z);
}

/// 生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短。'a 是大多数人默认使用的名称。
/// 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。
/// 泛型生命周期参数需要声明在函数名和参数列表间的尖括号中
/// &i32        引用
/// &'a i32     带有显式生命周期的引用
/// &'a mut i32 带有显式生命周期的可变引用
fn test2(){
    let s1=String::from("s1");
    let s2="s2";

    /// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
    /// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
    /// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
    fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
        if x.len()> y.len() {
            x
        }else{
            y
        }
    };
    let result=longest(s1.as_str(),s2);//返回的对象的生命周期是s1和s2中最小的那个
    println!("{},{}",s1,s2);
    println!("result:{}",result);
}

///拥有不同声明周期的值调用函数
fn test3(){
    fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
        if x.len()> y.len() {
            x
        }else{
            y
        }
    };
    let s1=String::from("a1");//
    {
        let s2=String::from("a2");
        let result=longest(s1.as_str(),s2.as_str());//返回的对象的生命周期和s2相同
        println!("{},{}",s1,s2);
        println!("result:{}",result);
    }
}

/// 错误的声明周期引用
fn test3_error(){
    ///longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
    fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
        if x.len()> y.len() {
            x
        }else{
            y
        }
    };
    let s1=String::from("b1");
    let result;
    let result2;
    {//s2声明周期范围
        let s2=String::from("b2");
        result=longest(s1.as_str(),s2.as_str());//返回的对象引用的生命周期和s2相同

        //上面的result赋值等同于下面2行,把一个s2生命周期的引用传递给了s1
        let tmp=longest(s1.as_str(),s2.as_str());
        result2=tmp;

        println!("{},{}",s1,s2);
    }//离开s2作用域后,result指向的值生命周期结束
    //println!("result:{}",result); //error 引用一个空值
    //println!("result2:{}",result2); //erro 引用一个空值
}

///
fn test4_error(){
    ///当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
    /// 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。
    fn longest<'a>(x:& str,y:& str)->&'a str{
        // let result = String::from("really long string");
        // result.as_str()
        ""
    };
    let s1=String::from("qq");
    let s2=String::from("ww");
    let result=longest(s1.as_str(),s2.as_str());
    println!("{},{}",s1,s2);
    println!("result:{}",result);
}


///结构体生命周期定义
fn test5(){
    struct ImportantExcerpt<'a ,T,T2:Display> {
        part: &'a str,
        part2: T,
        part3: T2,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence,part2:String::new() ,part3:String::new()};
}

///方法定义中的生命周期
fn test6(){
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a>{
        ///通过省略规则一和二,(每一个是引用的参数都有它自己的生命周期参数+只有一个输入生命周期参数,返回值和它的生命周期相同)
        fn level(&self)->i32{
            3
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        ///通过省略规则三:如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)那么所有输出生命周期参数被赋予 self 的生命周期。
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

///静态生命周期
/// 'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
fn test7(){
    let s: &'static str = "I have a static lifetime.";
    //这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面值都是 'static 的。

}

///在同一函数中指定泛型类型参数、trait bounds 和生命周期的语法！
fn test8(){
    ///当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
    ///显示指定声明周期,因为其不满足省略规则
    fn demo<'a,T>(x:&'a str,y:&'a str,ann:T)->&'a str where T:Display{
        println!("ann:{}",ann);
        if x.len() > y.len(){
            x
        }else {
            y
        }
    }
}


