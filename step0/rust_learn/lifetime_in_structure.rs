struct A<'a>{
    name:&'a str,
}

fn main(){
    let n = String::from("Hello");
    let a =A{name:&n};
    println!("{}",a.name);
}