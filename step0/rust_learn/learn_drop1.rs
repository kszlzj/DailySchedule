struct Dog{
    name:String,
}

impl Drop for Dog{
    fn drop(&mut self){
        println!("{} is leave",self.name);
    }
}
//rust
fn main(){
    let a= Dog{name:String::from("asd")};
    drop(a);
}