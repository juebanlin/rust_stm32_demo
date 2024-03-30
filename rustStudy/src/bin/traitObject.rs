trait Animal{
    fn speak(&self);
}

struct Cat;
impl Animal for Cat{
    fn speak(&self) {
        println!("喵！！！");
    }
}

struct Dog{
 name: String,
}
impl Dog {
    fn new(name_: &str)->Self{
        let s=name_.to_string();
        Dog{
            name:s,
        }
    }
}
impl Animal for Dog{
    fn speak(&self) {
        println!("{}:汪汪!",self.name);
    }
}

fn animals_speak(animals:Vec<Box<dyn Animal>>){
    for animal in animals {
        animal.speak();
    }
}

fn test1(){
    println!("######test1");
    let cat=Cat;
    let dog=Dog::new("dog0");
    let cat=&cat as &dyn Animal;
    let dog=&dog as &dyn Animal;
    cat.speak();
    dog.speak();
    //borrow
    let cat1:&dyn Animal =&Cat;
    let dog1:&dyn Animal =&Dog::new("dog1");
    cat1.speak();
    dog1.speak();
    //move
    let cat2:Box<dyn Animal>=Box::new(Cat);
    let dog2:Box<dyn Animal>=Box::new(Dog::new("dog2"));
    cat2.speak();
    dog2.speak();
}

fn test2(){
    println!("######test2");
    let cat:Box<dyn Animal>=Box::new(Cat);
    let dog:Box<dyn Animal>=Box::new(Dog::new("dog"));
    let animals=vec![cat, dog];
    animals_speak(animals);
    //写法2
    let cat1=Box::new(Cat);
    let dog1=Box::new(Dog::new("dog1"));
    let animals1:Vec<Box<dyn Animal>>=vec![cat1, dog1];
    animals_speak(animals1);
}

fn main() {
    test1();
    test2();
}


