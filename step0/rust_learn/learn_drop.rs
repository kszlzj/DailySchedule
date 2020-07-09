struct Dod {
    name:String,
}

impl Drop for Dog{
    fn drop(&mut self){
        println!("Dog leave");
    }
}

fn main(){
    let a = Dog{name:String::from("wangcai")};
    {
        let b = Dog{name:String::from("dahuang")};
    }
}