use inline_python::pyo3::libc::free;

/**
函数参数的mut修饰都在变量的左边,
例如： mut a:String
而引用租借修饰&都在:右边
例如： mut a:&mut String

注意:
变量左边的修饰符是给当前函数使用的,
变量冒号右边的修饰是告诉外面调用者传入类型
**/

fn baseTypeAdd(mut a: i32,mut b:i32){
    a=a+b;
}

fn main() {
    test14();
}

fn test14(){
    let mut str1=String::from("1");
    let mut str2=String::from("2");
    let mut str3=String::from("3");
    test1(str1);//移动到函数
    test2(str2);//移动到函数
    test3(&str3);//不可变借用
    test4(&mut str3);//可变借用
    test4(&mut str3);//可变借用
}

fn test1(str: String){
    println!("{}",str);
    //str="".to_string();//参数不可修改
    //str.push_str("");//参数内部不可修改
}

fn test2(mut str: String){
    println!("{}",str);
    str="".to_string();//参数可修改
}

fn test3(str:& String){
    println!("{}",str);
}

fn test4(str:&mut String){
    println!("{}",str);
    str.push_str("");//参数内部可修改
    // str="".to_string();//参数不可修改
}

fn test5(mut str:&mut String){
    println!("{}",str);
    str.push_str("");//参数内部可修改
}

