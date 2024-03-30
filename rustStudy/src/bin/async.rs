#![feature(thread_id_value)]

use futures::executor::{block_on};
use async_std::task::{sleep, spawn};
use std::{thread, time};

async fn learn_song() -> String {
    let ten_millis = time::Duration::from_millis(1000);
    for i in 1..4{
        let tid=thread::current().id().as_u64();
        println!("thread:{} learning song ……{}",tid,i);
        sleep(ten_millis).await
    }
    String::from("稻香")
}
async fn sing_song(song: String) {
    let ten_millis = time::Duration::from_millis(1000);
    let tid=thread::current().id().as_u64();
    for i in 1..4{
        println!("thread:{},sing:{} step:{}",tid,song.as_str(),i);
        sleep(ten_millis).await
    }
}
async fn dance(index:i32) {
    let ten_millis = time::Duration::from_millis(1000);
    let tid=thread::current().id().as_u64();
    for i in 1..4{
        println!("thread:{},dance type{} step:{}",tid,index,i);
        sleep(ten_millis).await
    }
}

fn test1(){
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance(0));
}

async fn test2(){
    // 要唱歌必须得先学会歌曲.
    // 我们这里使用 `.await` 而不是 `block_on` 来
    // 防止线程阻塞, 这样也可以同时跳舞.
    let song = learn_song().await;
    let f1 = sing_song(song);
    // `join!` 类似 `.await`，但是可以同时等待多个 `future` 执行完成.
    // 如果我们 `learn_and_sing` 这个 `future` 被阻塞, 那么 `dance`
    // 这个 `future` 将接管当前的线程. 如果 `dance` 被阻塞, 那么 `learn_and_sing`
    // 就可以重新开始. 如果这个两个 `future` 都被阻塞, 那么 `async_main`
    // 也将被阻塞并让位给执行程序.
    futures::join!(f1, dance(0),dance(1),dance(2),dance(3),dance(4));
}


fn main() {
    block_on(test2());
}
