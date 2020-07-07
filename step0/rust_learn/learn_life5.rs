use std::fmt::Display;
fn function<'a,T:Display>(x:&'a str,y:&'a str,ann:T)->&'a str{
    println!("ann= {}",ann);
    if x.len()<y.len(){
        x
    }else{
        y
    }
} 
fn main(){
    let s1=String::from("i am s1");
    let s2=String::from("i am s2,hello");
    let ann = 129;
    let r = function(s1.as_str(), s2.as_str(), ann);
    println!("i'm {}",r);
}