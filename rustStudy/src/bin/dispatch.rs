#![feature(fn_traits)]
use std::collections::HashMap;
use protobuf::well_known_types::Type;
use std::env::Args;
use std::fmt::{Display, Formatter};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr::NonNull;
use std::borrow::BorrowMut;

#[derive(Clone,Debug)]
enum Event{
    Click,
    Press,
}

impl Display for Event{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s=match self {
            Event::Click => {"Click"}
            Event::Press => {"Press"}
        };
        write!(f,"{}",s);
        Result::Ok(())
    }
}

fn test1(){
    fn click(event:Event)->(){
        println!("on click::{}", event);
    }
    fn press(event:Event)->(){
        println!("on press::{}", event);
    }
    type FnType =dyn Fn(Event) ->();
    let mut map=HashMap::<String,Box<FnType>>::new();
    map.insert("click".to_string(),Box::new(click));
    map.insert("press".to_string(),Box::new(press));
    for x in map.values() {
        x(Event::Click);
    }
}

struct Component{
    name:String,
    cb:Option<Box<dyn FnMut(&mut Component, Event) -> bool + 'static>>
}
impl Component {
    fn new(name:String) ->Self{
        Component{name:name,cb:None}
    }
}
trait IComponent{
    fn onClick(&mut self);
    fn onPress(&mut self);
    fn handle<F: FnMut(&mut Self, Event) -> bool + 'static>(&mut self, cb: F);
}

impl IComponent for Component{

    fn onClick(&mut self) {
        if self.cb.is_none() {
            return;
        }
        let mut cb=self.cb.take().unwrap();
        {
            (*cb)(self,Event::Click);
        }
        if self.cb.is_none() {
            self.cb=Some(cb);
        }
    }

    fn onPress(&mut self) {
        if self.cb.is_none() {
            return;
        }
        let mut cb=self.cb.take().unwrap();
        {
            (*cb)(self,Event::Press);
        }
        if self.cb.is_none() {
            self.cb=Some(cb);
        }
    }
    
    fn handle<F: FnMut(&mut Self, Event) -> bool + 'static>(&mut self,cb: F) {
        self.cb=Some(Box::new(cb));
    }
}

#[test]
fn test2(){
    let mut btn1=Component::new("button1".to_string());
    let mut btn2 =Component::new("button2".to_string());
    btn1.handle(move|c,e|{
        println!("{} on Event:{}",c.name,e);
        true
    });
    btn2.handle(move|c,e|{
        println!("{} on Event:{}",c.name,e);
        true
    });
    btn1.onClick();
    btn1.onPress();
    btn2.onClick();
    btn2.onPress();
}


fn main() {
    test2();

}

