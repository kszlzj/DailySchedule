// enum List{
//     Cons(i32,Box<List>)
//     Nil,
// }

enum List{
    Cons(i32,Rc<List>),
    Nil,
}

use std::rc::Rc;
use crate::List::{Cons,Nil};

fn main(){
    // let a = Cons(5,Box::new(Cons(10,Box::new(Nil))));
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    // let b = Cons(3,Box::new(a));
    let b = Cons(3,Rc::clone(&a));
    // let b = Cons(4,Box::new(a));
    let c = Cons(4,a.clone());
    println!("Hello,World!");

}