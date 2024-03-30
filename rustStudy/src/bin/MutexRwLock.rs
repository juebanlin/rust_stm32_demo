use std::sync::mpsc::channel;
use std::borrow::Borrow;
use std::sync::atomic::{AtomicPtr, Ordering};

/**
Mutex 意为互斥对象，用来保护共享数据。
在多线程中，Mutex 一般和 Arc 配合使用。
**/

fn main() {
    let (a,b)=(1i32,2i32);
    test1();
}

/// .lock() 方法，会等待锁令牌，等待的时候，会阻塞当前线程。而 .try_lock() 方法，只是做一次尝试操作，不会阻塞当前线程。
/// 当 .try_lock() 没有获取到锁令牌时，会返回 Err。因此，如果要使用 .try_lock()，需要对返回值做仔细处理（比如，在一个循环检查中）
fn test1(){
    use std::sync::{Arc,Mutex};
    use std::thread;
    let data=Arc::new(Mutex::new(0));
    let (tx,rx) = channel();
    for _ in 0..5{
        let data=data.clone();
        let tx=tx.clone();
        thread::spawn(move||{
            //拿到锁
            let mut data=data.lock().unwrap();
            *data+=1;
            if *data == 3{
                tx.send(()).unwrap();//发送信号
            }
        });
    }
    rx.recv().unwrap();//等待接收,类似stdin等待线程退出
    println!("{}",data.lock().unwrap());//打印值
}

fn test2(){
    use std::sync::RwLock;
    let lock=RwLock::new(5);
    {
        let r1=lock.read().unwrap();
        let r2=lock.read().unwrap();
        println!("{}",r1);
        println!("{}",r2);
    }
    {
        let mut w=lock.write().unwrap();
        *w+=1;
        println!("{}",*w);
    }
}

fn test3(){
    let mut str ="".to_string();
    let mut ato =AtomicPtr::<String>::new(&mut str);
    let mut str2="12".to_string();
    ato.compare_exchange(&mut str, &mut str2, Ordering::Relaxed, Ordering::Relaxed);

    let x = ato.get_mut();
    let y=*x;
}