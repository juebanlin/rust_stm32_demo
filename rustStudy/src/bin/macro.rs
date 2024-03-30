
/**
相比函数，宏是用来生成代码的，在调用宏的地方，编译器会先将宏进行展开，生成代码，然后再编译展开后的代码。
参数可以用圆括号(())、花括号({})、方括号([])中的任意一种括起来，
比如这行也可以写成 println!["Hello, world!"] 或 println!{"Hello, world!"}，
不过对于 Rust 内置的宏都有约定俗成的括号，比如 vec! 用方括号，assert_eq! 用圆括号。

宏定义里面的变量都是以 $ 开头的，相应的类型也是以冒号分隔说明，
这里 ident 是变量 $func_name 的类型，表示这个变量是一个 identifier，
这是语法层面的类型(designator)，而普通的类型如 char, &str, i32, f64 这些是语义层面的类型

宏里面的变量都是以 $ 开头的，其余的都是按字面去匹配，以 $ 开头的变量都是用来表示语法(syntactic)元素，
宏的body部分 可以是 圆括号、方括号、花括号中的任意一种
**/

macro_rules! create_function {
    ($fun_name:ident) => (
        fn $fun_name(){
            println!("fun {:?} is called,",stringify!($fun_name));
        }
    );
}

/**
为了限定匹配什么类型的语法元素，需要用指示符(designator)加以限定，就跟普通的变量绑定一样用冒号将变量和类型分开，当前宏支持以下几种指示符：
ident: 标识符，用来表示函数或变量名
expr: 表达式
block: 代码块，用花括号包起来的多个语句
pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
tt: 单个语法树
ty: 类型，语义层面的类型，如 i32, char
item: 条目，
meta: 元条目
stmt: 单条语句，如 let a = 42;
**/
macro_rules! muffin {
    ($x:ident) => {
        println!("arg is {}",$x);
    };
    ($x:expr) => {
        println!("arg is {}",$x);
    };
}

/**
宏相比函数一个很大的不同是宏可以接受任意多个参数
与正则表达式一样， + 表示一次或多次（至少一次），而 * 表示零次或多次。
重复的模式需要用括号括起来，外面再加上 $，例如 $(...),*, $(...);+。
重复的模式是用逗号或分号分隔的 allowed there are: `=>`, `,` or `;`
**/
macro_rules! myvec {
    //匹配零次或多次用逗号分隔的表达式
    ($($v:expr),*) => {
        //生成一个代码块CONTENT,代码块具有返回值,编译器把myvec!(1,2,3);替换为CONTENT;
        {
            let mut tmp_vec=Vec::new();
            $(tmp_vec.push($v);)*//重复的模式,类似for while
            tmp_vec
        }
    };
}

/**
除了重复之外，宏还支持递归，即在宏定义时调用其自身，类似于递归函数。
**/

macro_rules! find_max {
    //单值
    ($v:expr) => {
        $v
    };
    //多值,第一个后面是一个以上的重复参数
    ($x:expr,$($y:expr),+)=> {
        {
            let mut max=find_max!($($y),+);
            std::cmp::max($x,max)
        }
    }
}

/**
卫生宏最开始是由 Scheme 语言引入的，后来好多语言基本都采用卫生宏，
即编译器或运行时会保证宏里面定义的变量或函数不会与外面的冲突，
在宏里面以普通方式定义的变量作用域不会跑到宏外面。
**/

macro_rules! defina {
    () => ( let xx = 3; );
}
macro_rules! definb {
    ($v:ident) => { let $v=4; };
}
macro_rules! definc {
    () => {
        fn zz(){
            println!("zz");
        }
    };
}

fn main(){
    create_function!(myprint);
    myprint();
    let a=42;
    muffin!(a);
    muffin!(43i32);
    let vec0={
        vec![1,2,3]
    };
    let vec1=myvec!(1,2,3); //myvec!(1,2,3) 等同于{ vec![1,2,3] }
    // let vec2=myvec!(4=>5=>6);
    // let vec3=myvec!(7;8;9);
    println!("vec0:{:?}",vec0);
    println!("vec1:{:?}",vec1);
    println!("vec2:{:?}",vec2);
    println!("vec3:{:?}",vec3);

    println!("max:{:?}",find_max!(1,2,3,4,5));

    defina!();
    //println!("{}",xx);//变量 xx 是按普通方式定义的，所以其作用域限定在宏里面，宏调用结束后再引用 x 编译器就会报错。

    definb!(yy);//这种定义的变量,结束后仍然有效
    println!("{}",yy);

    //函数在宏里面以普通方式定义后，宏调用之后，这个函数依然可用
    definc!();
    zz();
}


/**
宏导入导出用 #[macro_use] 和 #[macro_export]
父模块中定义的宏,其下的子模块是可见的，
要想子模块中定义的宏在其后面的父模块中可用，需要使用 #[macro_use]
**/

mod test{
    macro_rules! m1 { () => (()) }

    #[macro_export] //让test之外的模块可用
    macro_rules! m2 { () => (()) }

    #[macro_use] //子模块的宏让其父模块可用
    mod foo{
        macro_rules! m3 { () => (()) }
    }
    mod bar{
        #[macro_export] //让test之外的模块可用
        macro_rules! m4 { () => (()) }
    }

    //宏m1 m2 m3可以在这里使用
}

/*

//其它模块
mod test2{
    #[macro_use]
    extern crate test;
    // test 中 m2 m4可用
}
//其它模块
mod test3{
    #[macro_use(m4)] //只导入某个宏
    extern crate test;
    // test 中 m4可用
}

*/