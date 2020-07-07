pub trait GetInformation {
    fn get_name(&self)->&String;
    fn get_age(&self)->u32;
}

pub trait SchoolName{
    fn get_school_name(&self)->String {
        String::from("hongxingschool")
    }
}
pub struct Student{
    pub name:String,
    pub age: u32,
}



pub struct Teacher{
    pub name:String,
    pub age:u32,
    pub subject:String,
}



impl GetInformation for Student{
    fn get_name(&self)->&String{
        &self.name
    }
    fn get_age(&self)->u32{
        self.age
    }
}

impl GetInformation for Teacher{
    fn get_name(&self)->&String{
        &self.name
    }
    fn get_age(&self)->u32{
        self.age
    }
}

fn print_infomation(item: impl GetInformation){
    println!("name ={}",item.get_name());
    println!("age={}",item.get_age());
}

impl SchoolName for Teacher{}

fn main(){
    let s=Student{name:String::from("lalala"),age:10};
    let t=Teacher{name:String::from("lalala"),age:30,subject:String::from("math")};
    println!("student,name={}mage={}",s.get_age(),s.get_age());
    println!("teacher,name={}mage={}",t.get_age(),t.get_age());
    let t_school_name = t.get_school_name();
    println!(".....{}",t_school_name);
    print_infomation(s);
    print_infomation(t);
    println!("Hello,world");
}