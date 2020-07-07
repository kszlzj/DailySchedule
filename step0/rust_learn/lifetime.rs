fn longer<'a>(s1:&'a str,s2:&'a str)->&'a str{
    if s2.len()>s1.len(){
        s2
    }else{
        s1
    }
}
fn main(){
    let r;
    {
        let s1="rust";
        let s2="ecmascript";
        r=longer(s1,s2);
        println!("{}",r);
    }
}