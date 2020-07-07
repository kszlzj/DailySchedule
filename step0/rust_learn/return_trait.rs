//使用trait bound
trait GetName{
    fn get_name(&self) ->&String;
}

trait GetAge{
    fn get_age(&self)->u32;
}

struct PeopleMathInformation<T,U>{
    master:T,
    student:U,
}

impl<T:GetAge+GetName,U:GetAge+GetName> PeopleMathInformation<T,U>{
    fn print_all_information(&self){
        println!("{}",self.master.get_name());
        println!("{}",self.master.get_name());
        println!("{}",self.student.get_name());
        println!("{}",self.student.get_name());
    }
}
struct Teacher{
    name:String,
    age:u32,
}

impl GetName for Teacher{
    fn get_name(&self) -> &String{
        &(self.name)
    }
}

impl GetAge for Teacher{
    fn get_age(&self) -> u32{
        self.age
    }
}

struct Student{
    name:String,
    age:u32,
}

impl GetName for Student{
    fn get_name(&self) -> &String{
        &(self.name)
    }
}

impl GetAge for Student{
    fn get_age(&self) -> u32{
        self.age
    }
}

fn main(){
    let s = Student{name:"xioaming".to_string(),age:15};
    let t = Student{name:"xiauhua".to_string(),age:30};
    let m = PeopleMathInformation{master:t,student:s};
    m.print_all_information();
    println!("hello word");
}