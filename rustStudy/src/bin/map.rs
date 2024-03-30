use std::collections::HashMap;
use std::rc::Rc;
use futures::task::SpawnExt;
use std::sync::mpsc::channel;

fn main() {
    test3();
}

fn test1(){
    //创建map
    let mut map=HashMap::new();
    map.insert("k1",String::from("v1"));
    map.insert("k2",String::from("v2"));

    let key="k3";
    let v3=String::from("v3");
    map.insert(key,v3);//v3 String类型拥有所有权的值被移动到了map

    //在键没有对应值时插入
    map.entry("k4").or_insert(String::from("v4"));

    //转移到不可变引用
    let map2=map;
    //打印
    println!("{}",key);// key实现了copy trait,其值拷贝进了map
    //println!("{}",v3);//error 所有权被移动到了map,不能再次读取
    //println!("{}",map.get("k1").unwrap());//error map已经移动了
    println!("{}",map2.get("k1").unwrap());

    //遍历map
    for (k,v) in &map2{
        println!("key->v:{}->{}",k,v);
    }
    
    println!("{:?}",map2);

}

fn test2(){
    let mut map:HashMap<_,_>=[
        ("k1".to_string(),"v1".to_string()),
        ("k2".to_string(),"v2".to_string())
    ].into();


    let mut map2=HashMap::new();

    for key in map.keys(){
        map2.insert(key.as_str(),map.get(key).unwrap());
    }

    println!("{:?}",map2);
}

fn test3(){
    let mut vec=(1..3).collect::<Vec<_>>();
    println!("{:?}",vec);
}



