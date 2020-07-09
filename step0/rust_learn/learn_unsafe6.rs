//不安全的trait
//但至少有一个方法有编译器不能验证的不变量时，该trait就是不安全的
unsafe trait foo{
    fn foo(&self);
}
struct Bar();
unsafe impl Foo for Bar{
    fn foo(&self){
        println!("foo");
    }
}

fn main(){
    let a = Bar();
    a.foo();
    println!("Hello,world");
}