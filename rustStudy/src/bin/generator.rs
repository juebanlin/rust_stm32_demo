#![feature(generators)]
#![feature(generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let mut closure=||{
      println!("1");
      println!("2");
      println!("3");
      println!("4");
        return 4;
    };
    let mut gen=||{
        println!("1");
        yield 1;
        println!("2");
        yield 2;
        println!("3");
        yield 3;
        println!("4");
        yield 4;
    };
    let r=Pin::new(&mut gen).resume(());
    println!("{:?}",r);
    let r=Pin::new(&mut gen).resume(());
    println!("{:?}",r);
    let r=Pin::new(&mut gen).resume(());
    println!("{:?}",r);
    let r=Pin::new(&mut gen).resume(());
    println!("{:?}",r);

    let r=closure();
    println!("{:?}",r);
}