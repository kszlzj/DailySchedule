use std::ops::Deref;
enum List{
    Cons(i32,Box<List>),
    Nil,
}

struct MyBox(T);
impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl Deref for MyBox<T>{
    type Traget =t;
    fn deref(&self)->&T{
        &self.0
    }
}
use ::List::{cons,Nil};
fn main(){
    let b=Box::new(5);
    println!("b={}",b);
    let list=Cons(1,
        Box::new(2,
            Box::new(3,
                Box::new(Nil)))