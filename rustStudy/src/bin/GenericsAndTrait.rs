use std::fmt::{Display, Debug, Formatter};
use std::cmp::Ordering;
use std::borrow::Borrow;

///泛型和trait
/// rust的泛型和trait是编译器的优化实现,编译器会吧语法糖还原成代码,所以效率并不会受到影响

fn main() {
    test4();
}

///使用指定类型函数解决查找同类型列表的最大值
fn test1(){
    let list=vec![12,13,41,37,123];
    let list2=vec![112,13,141,37,13];

    fn find_max(list:&[i32])->i32{
      let mut v=list[0];
        for &x in list {
            if x > v {
                v= x;
            }
        }
        return v;
    };
    let v1=find_max(&list);
    let v2=find_max(&list2);
    println!("v1:{},v2:{}",v1,v2);
}

///结构体泛型
fn test2(){
    //x y类型必须相同
    struct Point<T>{
        x :T,
        y :T,
    }
    let point=Point{x:1,y:2};
    let float=Point{x:1.1,y:2.2};

    //使用2个泛型处理2个类型
    struct Loc<T,U>{
        x :T,
        y :U,
    }
    let point=Loc{x:1,y:2};
    let float=Loc{x:1,y:2.2};

    //枚举泛型
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    //方法泛型-为Point结构体添加方法
    impl <T> Point<T>{
        fn get_x(&self)->&T{
            &self.x
        }
    }
    //添加Point特定类型f32的方法
    impl Point<f32>{
       fn get_distance(&self)->f32 {
           (self.x.powi(2)+self.y.powi(2)).sqrt()
       }
    }
    //多泛型结构体添加方法
    impl<T,U> Loc<T,U> {

        fn mixup<A,B>(self,other: Loc<A,B>) -> Loc<T,B> {
            let x=self.x;
            let y=other.y;
            let loc=Loc {
                x,
                y
            };
            return loc;
        }
    }
    //实现了特定trait的类型才添加方法
    //当LOC的泛型满足此条件时,此方法才有
    impl<T: Display+PartialOrd,U:Clone> Loc<T,U>{
        fn som_fun(){

        }
    }
}


/// trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。
/// impl TRAIT for XXXX(enum struct)
/// 必须是公有 trait 才能使得其他 crate 可以实现它,即需要pub修饰的
fn test3(){
   let a= Box::<u8>::from(1);//troubfish
    let b:Box<&str>=Box::from("");//类型匹配
    let c:Box<T>=Box::from("");//T可以代表
    // let d:Box<u8>=Box::from("");//error 明显不匹配
    struct Point{
        x :i32,
        y :i32,
    }
    struct Loc{
        x :f32,
        y :f32,
    }
    //定义trait(接口特性)
    trait ToString{
        fn to_string(&self)->String;
    }

    //实现trait
    impl ToString for Point{
        fn to_string(&self) -> String {
            format!("x:{},y:{}",self.x,self.y)
        }
    }
    impl ToString for Loc{
        fn to_string(&self) -> String {
            format!("x:{},y:{}",self.x,self.y)
        }
    }

    //trait和函数返回类型配合
    fn print0(obj: impl ToString)->impl Display{
        let mut str=obj.to_string();
        println!("{}",&str);
        return str;
    }

    //trait和函数配合
    fn print1(obj: impl ToString){
        println!("{}",obj.to_string());
    }
    print1(Point{x:1,y:2});

    //trait和泛型配合
    fn print2<T: ToString>(obj:T){
        println!("{}",obj.to_string());
    }
    print2(Point{x:3,y:4});

    //多个trait和泛型配合
    impl Clone for Point{
        fn clone(&self) -> Self {
            let x=self.x;
            let y=self.y;
           return Point{x,y};
        }
    }
    fn print3<T: ToString + Clone>(obj:T){
        println!("{}",obj.to_string());
    }
    print3(Point{x:5,y:6});

    //推荐 通过where简化多个trait 和泛型绑定
    fn fun<T,U>(t:T,u:U) ->i32 where T: Display + Clone,U:Clone + Debug{
        return 0;
    }

    /// 可以对任何实现了特定trait的类型实现trait
    impl <T:Display> ToString for T{
        fn to_string(&self) -> String {
            todo!()
        }
    }
}

///trait和泛型搭配使用
fn test4(){
    ///方式1,使用不同的函数
    fn find_max_i32(list:&[i32])->i32{
        let mut v=list[0];
        for &x in list {
            if x > v {
                v= x;
            }
        }
        return v;
    };
    fn find_max_char(list:&[char])->char{
        let mut v=list[0];
        for &x in list {
            if x > v {
                v= x;
            }
        }
        return v;
    };
    let list=vec![12,13,41,37,123];
    let list2=vec!['a','b','m','q','s'];

    let v1=find_max_i32(&list);
    let v2=find_max_char(&list2);
    println!("v1:{},v2:{}",v1,v2);

    ///方式二 抽象特性,类似java中,集合存放的对象具有compareAble接口特性
    /// T 要实现 > 运算符比较特性才行
    fn find_max<T>(list: &[T])->T where T: PartialOrd+Copy{
        let mut v=list[0];
        for &x in list {
            if x > v{
                v=x;
            }
        }
        return v;
    };
    let v3=find_max(&list);
    let v4=find_max(&list2);
    println!("v3:{},v4:{}",v3,v4);
}