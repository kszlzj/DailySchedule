//4.解引用裸指针
//不可变的，可变的，分别写作*const T,*mut T

//
//(1)允许忽略借用规则
//(2)不保证指向有效的内存
//(3)允许为空
//(4)不执行自动清理

fn main(){
    let mut num=5;
    //创建不可变和可变的裸指针，不能在不安全快范围之外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe{
        println!("{}||{}",*r1,*r2);
    }
    let add = 0x12345usize;
    let r = add as *const i32;
     
}
