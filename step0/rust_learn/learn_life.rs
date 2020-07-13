// 示例：
// ```
// use std::fmt::Display;
// fn function<'a,T:Display>(x:&'a str,y:&'a str,ann:T)->&'a str{
//     println!("ann= {}",ann);
//     if x.len()<y.len(){
//         x
//     }else{
//         y
//     }
// } 
// fn main(){
//     let s1=String::from("i am s1");
//     let s2=String::from("i am s2,hello");
//     let ann = 129;
//     let r = function(s1.as_str(), s2.as_str(), ann);
//     println!("i'm {}",r);
// }
// ```
// 还看了闭包，闭包有以下知识点：

// 1.闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数。闭包和函数不同的是，闭包允许捕获调用者作用域中的值。2.闭包的使用方式3.使用带有泛型和fn trait的闭包
// 示例：
// ```
// fn main(){
//     let use_closure=||{
//         println!("This is a closure");
//     };
//     use_closure();
//     println!("Hello,world");
// }
// ```
// 语法格式与函数的区别:

// 函数：
// ```
//     fn add_one_v1(x:u32) ->u32{
//         x+1
//     }
// ```
// 闭包：
// ```
//     let add_one_v2=|x:u32|->u32{x+1};
//     let add_one_v3=|x|{x+1};
//     let add_one_v3=|x| x+1;
// ```
// 注意：

// 闭包回味每个参数和返回值推导一个具体的类型，但是不能推导两次

// 例如以下的代码就是错误的:

// ```
//     let example closure = |x| x;
//     let s= example_closire(String::from("hello"));
//     let n= example_closure(5);
// ```

// 捕捉环境中的变量
// ```
// let i=1;
// let exe=|x|x+i;
// let r = exe(5);
// println!("r={}",r);
// ```

// 下面使用闭包实现一个缓存，只处理第一次传入的值并保存

// ```
// struct Cacher<T>
//     where T: Fn(u32) -> u32
//     {
//         calculation:T,
//         value:Option<u32>,
//     }


// impl <T> Cacher<T>
//     where T:Fn(u32)->u32{
//         fn new(calculation:T)->Cacher<T>{
//             Cacher{
//                 calculation,
//                 value:None,
//             }
//         }
//         fn value(&mut self,arg:u32)->u32{
//             match self.value{
//                 Some(v)=>v,
//                 None=>{
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }

// fn main(){
//     let mut c=Cacher::new(|x| x+1);
//     let v1 = c.value(1);
//     println!("v1={}",v1);
// }
// ```
struct StuA<'a>{
    name:&'a str,
}

impl <'a> StuA<'a>{
    fn do_something(&self) -> i32{
        3
    }
    fn do_something2(&self,s:&str) -> &str{
        self.name
    }
    fn do_something3<'b>(&self,s:&'b str) -> &'b str{
        s
    }
}

fn main(){
    let s = String::from("hello");
    let a = StuA{name:&s};
    println!("{}",a.do_something());
    println!("{}",a.do_something2(&s));
    println!("{}",a.do_something3(&s));
    println!("Hello,world");
}