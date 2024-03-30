

fn main() {
    cmd();
    readfile();
    test2();
}

fn cmd(){
    //获取命令行参数,第一个是程序运行的名称
    let args:Vec<String> = std::env::args().collect();
    println!("{:?}",args);

    let name=&args[0];
    println!("{:?}",name);
}

fn readfile(){
    let filename="1.txt";
    let contents= std::fs::read_to_string(filename).expect("can not read");
    println!("{}",contents);
}

fn test2(){
    //ref用在变量绑定
    let ref a=2;
    let ref a2=&2;
    //&用在表达式
    let a = &2;

    //&用在绑定和*用在表达式效果一样
    let &c=a;
    let d=*a;

    //类型声明
    struct Point<'a>{
        x:&'a i32,
        y:i32
    }
    fn add(a:&i32,ref b:i32){

    }
    add(&5,3);
}