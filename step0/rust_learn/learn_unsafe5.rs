// static HELLO_WORLD:&str = "Hello,world";

// fn main(){
//     println!("{}",HELLO_WORLD);
// }

//静态变量有一个固定的值，常年允许被用到的时候复制器数据。
//静态变量可以是可变的（用unsafe）

static mut COUNTER:u32 =0;
fn add_counter(inc:u32){
    unsafe{
        COUNTER+=inc;
    }
}

fn main(){
    add_counter(3);
    unsafe{
        println!("{}",COUNTER);
    }
}
