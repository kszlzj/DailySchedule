//只读的共享数据，不然会竞争或者不一致
enum List{
    Cons(i32,Rc<List>),
    Nil,
}


use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main(){
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}",Rc::strong_count(&a));
    let b = Cons(3,Rc::clone(&a));
    println!("count after creating a = {}",Rc::strong_count(&a));
    { 
        let b = Cons(4,Rc::clone(&a));
        println!("count after creating a = {}",Rc::strong_count(&a));
    }
    println!("count after creating a = {}",Rc::strong_count(&a));
    println!("Hello world")
}