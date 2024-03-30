use futures::executor::block_on;
use async_executor::Executor;

fn main() {
    test1();
}

fn test1(){
    trait Foo {
        fn foo(&self);
    }

    trait FooBar : Foo {
        fn foobar(&self);
    }

    #[derive(Debug)]
    struct Baz;
    println!("{:?}",baz);
    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }
    //FooBar的实现者也要同时实现Foo
    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }
}


fn test2(){
    let ex = Executor::new();
    let fu1=async {
      println!("async block1");
    };
    let fu2=async {
      println!("async block2");
    };
    let fu3=async {
      println!("async block3");
    };
    block_on(fu1);
    block_on( ex.run(fu2));
    let task=ex.spawn(fu3);
}