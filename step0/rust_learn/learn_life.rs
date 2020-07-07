struct StuA<'a>{
    name:&'a str,
}

impl <'a> StuA<'a>{
    fn do_something(&self) -> i32{
        3
    }
    fn do_something2(&self,s:&str) -> &str{
        self.name
    }
    fn do_something3<'b>(&self,s:&'b str) -> &'b str{
        s
    }
}

fn main(){
    let s = String::from("hello");
    let a = StuA{name:&s};
    println!("{}",a.do_something());
    println!("{}",a.do_something2(&s));
    println!("{}",a.do_something3(&s));
    println!("Hello,world");
}