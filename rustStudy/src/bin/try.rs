use std::io;

fn main() {
    test().expect("读取失败");
}

///IO 操作基本上都是使用 io::Result<()>
fn test()->io::Result<()>{
    // let mut input1=String::new();
    let mut input2=String::new();
    // r#try!(std::io::stdin().read_line(&mut input1));
    io::stdin().read_line(&mut input2)?;
    let v=input2.trim().parse::<i32>().expect("转换失败");
    println!("{}",v);
    Ok(())//必须要有返回值
}