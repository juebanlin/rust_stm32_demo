use std::any::Any;

fn main() {
    let p=Point{x:1.1,y:2.2};
    println!("{:?}",p);
    let p2=Point::new(11f32,22f32);
    println!("{:?}",p2);
    let s=StatusV1::Run;
    let s2=StatusV2::Run;
    let s3=StatusV3::Run("1".to_string());
    match s3 {
        StatusV3::Idle(_) => {}
        StatusV3::Run(_) => {}
        StatusV3::Dead(_, _) => {}
    }
    match s2 {
        StatusV2::Idle => {}
        StatusV2::Run => {}
        StatusV2::Dead => {}
    }
    match s {
        StatusV1::Idle => {}
        StatusV1::Run => {}
        StatusV1::Dead => {}
    }
}

#[derive(Debug)]
struct Point{
    x:f32,
    y:f32,
}

impl Point{
    //添加构造函数
    fn new(x:f32,y:f32)->Point{
        Point{x,y}
    }
}

enum StatusV1{
    Idle,
    Run,
    Dead
}


//指定索引用于宏在执行序列化的时候查询需要写入的索引
enum StatusV2{
    Idle = 0,
    Run = 1,
    Dead = 2
}


enum StatusV3{
    Idle(i32),
    Run(String),
    Dead(i32,i32),
}

impl StatusV1{
    fn valueOf(value:i32)->Option<StatusV1>{
        match value {
            1=>Some(StatusV1::Idle),
            2=>Some(StatusV1::Run),
            3=>Some(StatusV1::Dead),
            _ => {None}
        }
    }
}