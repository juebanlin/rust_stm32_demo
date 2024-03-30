fn main() {
    test1();
}

fn test1(){
    let pid=std::process::id();
    println!("pid:{}",pid.to_string());
    let r=std::process::Command::new("ls").arg("-l").output().expect("cmd error");
    println!("{}",String::from_utf8_lossy(&r.stdout));
    println!("{}",String::from_utf8(r.stdout).unwrap());
    std::process::exit(-1);
}