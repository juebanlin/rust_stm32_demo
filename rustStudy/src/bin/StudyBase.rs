
use rustStudy;

fn main() {
    rustStudy::say_hello_world();//bin目录引用上层模块
    println!("###########");
    rustStudy::toolinfoV3();
    println!("###########");

    let mut a=1;//可变引用
    let b=2;//不可变引用
    a=3;
    a=a+b;
    println!("a:{}",a);

    fun1();
    fun2();
    fun3();
    fun4();
    fun5();
    fun6();
    fun7();
    fun9();
    fun10();

}

//函数体
fn fun1(){
    println!("#####fun1");
    let a=5;
    let b = {
      let x=1;
       x+1 //注意：x + 1 之后没有分号，否则它将变成一条语句！
        //函数体表达式并不能等同于函数体，它不能使用 return 关键字。
    };
    let c =b+1;
    println!("a:{},b{},c{}",a,b,c);
}

/// 函数定义嵌套和返回值
fn fun2(){
    println!("#####fun2");
    //在参数声明之后用 -> 来声明函数返回值的类型（不是 : ）
    //如果没有明确声明函数返回值的类型i32，函数将被认为是"纯过程"，不允许产生返回值
    fn add(a:i32,b: i32)->i32{
        a+b
    }
    fn sub(a:i32,b: i32)->i32{
        return a-b
    }
    fn add2(a:i32, b: i32) ->i32{
        return add(a,b)*2;
    }
    println!("add:{}",add(1,4));
    println!("sub:{}",sub(1,4));
    println!("addx2:{}", add2(1, 4));
}

fn fun3(){
    println!("#####fun3");
    let a=1;
    //条件表达式必须是 bool 类型，例如下面的程序是错误的
    //C/C++ 语言中的条件表达式用整数表示，非 0 即真，但这个规则在很多注重代码安全性的语言中是被禁止的。
    if a>0 {
        println!("a>0")
    }else{
        println!("a<0")
    }
    let b= if a>0 {1} else {-1};
    let c= {if a>0 {1} else {-1}};
    let d= {b+c};
    println!("b:{},c:{},d:{}",b,c,d);
}

fn fun4(){
    println!("#####fun4");
    //Rust 语言到此教程编撰之日还没有 do-while 的用法，但是 do 被规定为保留字，也许以后的版本中会用到
    //rust 没有fori,需要使用while代替
    let mut a=5;
    while a>0 {
        println!("a:{}",a);
        a-=1;
    }
    //范围循环
    for i in 0..5 {
        println!("i:{}",i);
    }
    let b=5;
    for j in 0..b {
        println!("j:{}",j);
    }
    //数组循环
    let array = [10, 20, 30, 40, 50];
    let mut index=0;
    for i in array.iter() {
        println!("array[{}]:{}", index,i);
        index+=1;
    }

    //无限循环
    let mut index=0;
    let array2 = [1, 2, 3, 4, 5];
    loop {
        if index >=array2.len(){
            break;
        }
        let value=array2[index];
        index+=1;
        if value % 2 == 0{
            //不打印偶数
            continue;
        }
        println!("array2[{}]:{}", index,value);
    }
}

///栈和堆-变量所有权
fn fun5(){
    println!("#####fun5");
    //栈中执行,a绑定的值复制给b,栈中2个5
    /**
    基本类型在栈中的移动是复制
    所有整数类型，例如 i32 、 u32 、 i64 等。
    布尔类型 bool，值为 true 或 false 。
    所有浮点类型，f32 和 f64。
    字符类型 char。
    仅包含以上类型数据的元组（Tuples）。
    **/
    let a=5;
    let b=a;

    //堆引用,y 都指向堆中的对象,x引用被丢弃,z为复制
    //为了确保安全，在给 y 赋值时 x 已经无效了。没错，在把 x 的值赋给 y 以后 x 将不可以再被使用
    let x=String::from("hello");
    let y=x;
    //将堆中的 "hello" 复制了一份，所以 y 和 z 都分别绑定了一个值，释放的时候也会被当作两个资源
    let z=y.clone();
    //println!("x:{}",x); //error :value borrowed here after move
    println!("y:{}",y);
    println!("z:{}",z);
}

///函数所有权
fn fun6(){
    println!("#####fun6");

    let a=5;
    fn print1(value:i32){
        println!("value:{}",value);
        //// 函数结束, 参数 是基本类型, 无需释放
    }
    print1(a);//基本数据类型是复制效果
    println!("a:{}",a);


    let s=String::from("hello");
    fn print2(str:String){
        println!("str:{}",str);
        // 函数结束, 参数在这里释放
    }
    print2(s);//将堆变量当作参数传入函数，那么它和移动的效果是一样的。
    //println!("{}",s);//error s已经被移动

    //返回值所有权
    //被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放

    let str=String::from("str");
    fn test(s:String)->String{
        return s;//所有权返回给函数调用者
    }
    let str2=test(str);//str2获得所有权
    println!("str2:{}",str2);
    //println!("str:{}",str);//error 所有权被移动无法打印
}

///所有权引用和租借
/// & 运算符可以取变量的"引用"。
/// 当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
/// 引用不会获得值的所有权。引用只能租借（Borrow）值的所有权。
/// 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权
fn fun7(){
    println!("#####fun7");
    let str=String::from("hello");
    fn test(s:String)->String{
        return s;//所有权返回给函数调用者
    }
    let str2=test(str);//str2获得所有权
    println!("str2:{}",str2);
    //println!("str:{}",str);//error 所有权被移动无法打印
    let mut str3=&str2;
    println!("引用str2的str3:{}",str3);//打印引用对象的值
    let str4=str2;
    println!("移动str2到str4:{}",str4);//注意此时所有堆str2的引用和租借都会失效,需要重新租借
    // println!("str3:{}",str3);//error str2的值已经被移动了,引用失效

    //重新租借
    str3=&str4;
    //引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子是一个道理）。
    // 如果尝试利用租借来的权利来修改数据会被阻止;
    //可变引用租借不可变变量后是进制修改租借的值
    //str3.push_str("hello2");//error 禁止修改租借的值

    //可变租借 使用&mut租借可变变量是可以修改租借的值
    let var1=String::from("var");
    fn brrow(mut arg: String) -> String {
        arg.push_str("+1");
        let  var2=&mut arg;
        var2.push_str("+2");
        var2.push_str("+3");
        // println!("var1:{}",arg);//error 多重租借
        println!("var2:{}",var2);
        return arg;
    };
    let var3=brrow(var1);
    println!("var3:{}",&var3);

    //多重可变租借
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    //println!("{}, {}", r1, r2);//error 多重租借

    //不可变租借和可变租借同时存在
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);//最后一次使用不可变引用在声明可变引用之前的话,就没问题
    let r3 = &mut s; //没问题
    println!("{}", r3);
    //后面不能使用r1和r2，因为r3可能会修改r1 r2的引用数据
}

///切片
fn fun8(){
    println!("#####fun8");
    let mut s=String::from("hello world");
    let a=&s[0..5];
    let b=&s[6..11];
    println!("a+b=a:{}+{}",a,b);

    s.push_str(" +1");//
    println!("s:{}",s);
    //切片引用的值修改后需要重新引用
    let a=&s[0..5];
    let b=&s[6..];
    println!("a+b=a:{}+{}",a,b);

    //字符串转换为str
    let s2=&s[..];
    println!("s2:{}",s2);

    //非字符串切片
    let array=[3;5];
    let subarray=&array[1..3];
    for i in subarray.iter(){
        println!("i:{}",i);
    }
}


///结构体和结构体方法函数
fn fun9(){
    println!("#####fun9");
    #[derive(Debug)]
    struct Student{
        name:String,
        age:i32
    }
    let name=String::from("张三");
    let age = 22;
    //实例化
    let s1=Student{
        name,age
    };
    //新建一个年龄相同的李四
    let s2=Student{
        name:String::from("李四"),
        ..s1//s1后面没有逗号,除了name之外的其它属性值都参考s1
    };

    //元祖结构体
    #[derive(Debug)]
    struct Color(u8,u8,u8);
    #[derive(Debug)]
    struct point(f64,f64);

    let black=Color(0,0,0);
    let origin=(0.0,0.0,0.0);
    //元组结构体对象的使用方式和元组一样，通过 . 和下标来进行访问
    println!("black:{} {} {}",black.0,black.1,black.2);
    println!("origin:{} {} {}",origin.0,origin.1,origin.2);

    //打印结构体
    println!("s1:{:?}",s1);
    println!("black:{:?}",black);

    //结构体方法
    //结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字
    struct Rectangle{
        width: u32,
        height: u32
    }

    //给结构体追加方法
    //结构体 impl 块可以写几次，效果相当于它们内容的拼接！
    impl Rectangle{
        //结构体方法
        fn area(&self )->u32{
            return self.width*self.height;
        }
        //结构体关联函数 类似static方法
        fn create(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
    let rect=Rectangle::create(10,20);
    println!("rect area:{}",rect.area())
}

///枚举和匹配
fn fun10(){
    println!("#####fun10");
    //枚举定义
    enum Type{
        A,
        B
    }
    let t=Type::A;

    enum Type2{
        A(String),
        B(i32)
    }
    let ta=Type2::A(String::from("ta"));
    let tb=Type2::B(111);

    enum Message{
        Quit,
        Move{x:i32,y:i32},
        Write(String),
        Color(i32,i32,i32)
    }

    //枚举可以像结构体一样定义方法
    impl Message{

        fn name(&self) ->String{
            match self{
                //枚举的匹配
                Message::Quit => String::from("Quit"),
                Message::Move { .. } => String::from("Move"),
                Message::Write(_) => String::from("Write"),
                Message::Color(_,_,_) => String::from("Color"),
                _ => String::from("default"),//通配符_ 类似case default
            }
        }

        fn index(&self) -> i32{
            match self {
                Message::Quit => 1,
                Message::Move {x, y }  => 2,
                Message::Write(_) => 3,
                Message::Color(_, _, _) => 4,
            }
        }

        fn print(&self){
            match self {
                //使用枚举值占位符
                Message::Quit => println!("value:"),
                Message::Move {x, y }  => println!("x:{},y{}",x,y),
                Message::Write(s) => println!("value:{}",s),
                Message::Color(v1, v2, v3) => println!("v1:{},v2{},v3{}",v1,v2,v3),
            }
        }

    }
    let m=Message::Write(String::from("hello"));
    m.print();
    println!("name:{}, index:{}",m.name(),m.index());

    //库中的自带Option枚举
    let x=5;
    let y:Option<i32> = Some(5);
    let sum=x+y.unwrap();
    println!("sum:{}",sum);

    //option值加一
    fn addOption(op:Option<i32>) -> Option<i32>{
        match op {
            None => {None},
            Some(i) =>{ Some(i+1)}
        }
    }
    let z=addOption(y);
    println!("z:{}",z.unwrap());

    //枚举嵌套
    enum Sex{
        Male,
        Fmail,
    }
    enum Worker{
        Teacher(Sex),
        Student(Sex)
    }

    //单一匹配 if let
    let tom=Worker::Student(Sex::Male);
    if let Worker::Student(Sex::Male) = tom {
        println!("tom is male student");
    }
}
