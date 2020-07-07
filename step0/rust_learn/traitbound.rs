// fn print_infomation(item: impl GetInformation){
//     println!("name ={}",item.get_name());
//     println!("age={}",item.get_age());
// }
//直接作为参数2写法
//-------------------------------------------------
//trat_bound 写法
// fn print_infomation<T:GetInformation>(item: T){
//     println!("name ={}",item.get_name());
//     println!("age={}",item.get_age());
// }
pub struct Student{
    pub name:String,
    pub age: u32,
}

#[derive(Debug)]
pub struct Teacher{
    pub name:String,
    pub age:u32,
}

impl GetAge for Teacher{
    fn get_age(&self)->u32{
        self.age
    }
}
trait GetName{
    fn get_Name(&self)->&String;
}

trait GetAge{
    fn get_age(&self) ->u32;
}

impl GetName for Student{
    fn get_Name(&self)->&String{
        &self.name
    }
}

impl GetAge for Student{
    fn get_age(&self)->u32{
        self.age
    }
}

//写法一
// fn print_infomation<T:GetName+GetAge>(item: T){
//     println!("name ={}",item.get_Name());
//     println!("age={}",item.get_age());
// }

//写法二
fn print_infomation<T>(item: T)
    where T:GetName+GetAge
{
    println!("name ={}",item.get_Name());
    println!("age={}",item.get_age());
}

fn produce_item_with_age()->impl GetAge{
    Student{
        name:"xiaoming".to_string(),
        age:15,
    }
}

//返回至一定是确定的

fn main()
{
    let s=Student{name:String::from("lalala"),age:10};
    print_infomation(s);
}