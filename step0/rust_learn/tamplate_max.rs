use std::cmp::PartialOrd;
fn largest<T:PartialOrd+Copy>(list:&[T])->T{
    let mut large =list[0];
    for &item in list.iter(){
        if item >large{
            large=item
        }
    }
    large
}

fn main(){
    let number = vec![35,50,25,100,65];
    let result =largest(&number);
    println!("{}",result);
    println!("{:?}",number);
}