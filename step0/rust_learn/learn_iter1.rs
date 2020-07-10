trait Iterator{
    type Item;
    fn next(mut self)->Option<Self::Item>;//type Item和Self::item叫做关联类型

}
fn main(){
    let v1=vec![1,2,3];
    let mut v1_iter=v1.iter();
    // for val in v1_iter{
    //     println!("val = {}",val);
    // }
    if let Some(v) = v1_iter.next(){
        println!("{}",v);
    }
    if let Some(v) = v1_iter.next(){
        println!("{}",v);
    }
    if let Some(v) = v1_iter.next(){
        println!("{}",v);
    }else{
        println!("end");
    }
    //--------------可变应用
    let mut v2 = vec![1,2,3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next(){
        *v = 3;
    }
    println!("{:?}",v2);
    //--------------消费适配器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let totle :i32= v1_iter.sum();
    println!("{}",totle);
    //--------------迭代适配器
    let v1 = vec![1,2,3];
    let v2 :Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}",v2);
    //--------------迭代适配器
    let v1 = vec![1,12,3,45];
    println!("{:?}",v1);
    let v2 :Vec<_> = v1.into_iter().filter(|x|*x>5).collect();
    println!("{:?}",v2);
}