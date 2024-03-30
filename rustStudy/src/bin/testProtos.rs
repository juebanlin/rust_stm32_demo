extern crate protobuf;
use std::borrow::BorrowMut;
use rustStudy::utils::math::add;
fn main() {
    it_adds_two();
    // let mut a =HelloWordRequest::new();
    // a.set_hot_word_text("test".to_string());
    // println!("{}",1);
    // println!("{:?}",&a);
    // println!("{:#?}",&a);
}



fn it_adds_two() {
    add(2,2);
}