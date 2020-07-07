fn longest<'a>(x: &'a str,y:&'a str) -> &'a str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}

fn a_str(&self,x:&str,y:&str)->&str{
    let r=String::from("abc");
    let res=r.as_str();
    res
}

fn main(){
    let s1 =String::from("aaaaaaaaaaaaa");
    let s2 =String::from("ac");
    let r=longest(s1.as_str(), s1.as_str());
    println!("{}",r);
    let ss2 = a_str(s1.as_str(),s2.as_str());
    println!("{}",ss2);
}