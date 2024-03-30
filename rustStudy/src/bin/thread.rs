#![feature(thread_id_value)]

use std::thread;
use std::sync::{Arc, RwLock};
use std::borrow::Borrow;
use std::thread::sleep;
use core::time;

fn main() {
    test();
    test3();
}

/**
线程创建方式
*/
fn test(){
    let t=thread::current();
    let id=t.id().as_u64().get();
    let name=t.name().unwrap_or_default();
    println!("thread,id:{},name:{}",id,name);
    let t1=thread::spawn(||{
        let t=thread::current();
        let id=t.id().as_u64().get();
        let name=t.name().unwrap_or_default();
        println!("t1 custom thread,id:{},name:{}",id,name);
    });
    t1.join().unwrap();
    let t2=thread::Builder::new().name("newThread".to_string()).spawn(||{
        let t=thread::current();
        let id=t.id().as_u64().get();
        let name=t.name().unwrap_or_default();
        println!("t2 custom thread,id:{},name:{}",id,name);
    }).unwrap();
    t2.join().unwrap();
}

/**
多线程访问只读共享数据
 */
fn test2(){
    let list=Arc::new((10..13).collect::<Vec<i32>>());
    for i in 1..3{
        let b=thread::Builder::new().name("t1".to_string());
        let tmp_list=list.clone();
        let t=b.spawn(move||{
            for x in tmp_list.iter() {
                println!("{}",x);
            }
        }).unwrap();
        t.join().unwrap();
    }
}

/**
多线程修改共享数据
*/
fn test3(){
    let list=Arc::new(RwLock::new((10..13).collect::<Vec<i32>>()));
    for i in 1..=3{
        let b=thread::Builder::new().name(format!("t:{}",i).to_string());
        let tmp_list=list.clone();
        let t=b.spawn(move||{
            if std::time::SystemTime::now().elapsed().unwrap().as_millis() % 2== 1{
                let lock=tmp_list.read().unwrap();
                for x in lock.iter() {
                    println!("{}",x);
                }
            }else {
                let mut lock=tmp_list.write().unwrap();
                lock[0]+=100;
                for x in lock.iter() {
                    println!("{}",x);
                }
            }
        }).unwrap();
        t.join().unwrap();
    }
    sleep( time::Duration::from_millis(1000));
}