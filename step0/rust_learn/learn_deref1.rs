use std::ops::Deref;
struct Mybox <T>(T);

impl <T> Mybox<T>{
    fn new(x:T) -> Mybox<T>{
        Mybox(x)
    }
}

impl<T> Deref for Mybox<T>{
    type Target = T;
    fn deref(&self)->&T{
        &self.0
    }
}

fn main(){
    let x=5;
    let y= Mybox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
}