# DailySchedule
OS Tutorial Summer of Code 2020 by 罗子健
---
## 7月4日
由于之间对deno(一个typescript运行时)项目感兴趣，抽了一些时间学习rust基础语法，所以没有从一开始的语法内容开始学习。

***P.S.我把学习中的代码示例放入了repo中step0/rust_learn文件夹。***


完成rustlings上[variable](https://github.com/kszlzj/DailySchedule/tree/master/step0/variables) [primitive_types](https://github.com/kszlzj/DailySchedule/tree/master/step0/primitive_types) [function](https://github.com/kszlzj/DailySchedule/tree/master/step0/functions) [if](https://github.com/kszlzj/DailySchedule/tree/master/step0/if) [test1](https://github.com/kszlzj/DailySchedule/blob/master/step0/test1.rs)章节
---
## 7月5日
将rustlings的进度推进到了error1.rs
---
## 7月6日
rustlings做到conversion的from_into.rs
---
## 7月7日
由于对生命周期的理解不够深入，今天我抽时间观看令狐壹冲Rust编程视频教程（进阶）中关于生命周期的讲解。
从生命周期的省略开始看，明白了生命周期省略的三条规则 ：

*1、每个引用的参数都有它自己的生命周期参数。*

*2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。*

*3、如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self或者&mut self，那么self的生命周期被赋予所有输出生命周期参数。*

静态生命周期：'static 其生命周期存活于整个程序期间，所有的字符字面量都拥有static生命周期。
`let s:&'static str ="hello";`
示例：
```
use std::fmt::Display;
fn function<'a,T:Display>(x:&'a str,y:&'a str,ann:T)->&'a str{
    println!("ann= {}",ann);
    if x.len()<y.len(){
        x
    }else{
        y
    }
} 
fn main(){
    let s1=String::from("i am s1");
    let s2=String::from("i am s2,hello");
    let ann = 129;
    let r = function(s1.as_str(), s2.as_str(), ann);
    println!("i'm {}",r);
}
```
还看了闭包，闭包有以下知识点：

1.闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数。闭包和函数不同的是，闭包允许捕获调用者作用域中的值。2.闭包的使用方式3.使用带有泛型和fn trait的闭包
示例：
```
fn main(){
    let use_closure=||{
        println!("This is a closure");
    };
    use_closure();
    println!("Hello,world");
}
```
语法格式与函数的区别:

函数：
```
    fn add_one_v1(x:u32) ->u32{
        x+1
    }
```
闭包：
```
    let add_one_v2=|x:u32|->u32{x+1};
    let add_one_v3=|x|{x+1};
    let add_one_v3=|x| x+1;
```
注意：

闭包回味每个参数和返回值推导一个具体的类型，但是不能推导两次

例如以下的代码就是错误的:

```
    let example closure = |x| x;
    let s= example_closire(String::from("hello"));
    let n= example_closure(5);
```

捕捉环境中的变量
```
let i=1;
let exe=|x|x+i;
let r = exe(5);
println!("r={}",r);
```

下面使用闭包实现一个缓存，只处理第一次传入的值并保存

```
struct Cacher<T>
    where T: Fn(u32) -> u32
    {
        calculation:T,
        value:Option<u32>,
    }


impl <T> Cacher<T>
    where T:Fn(u32)->u32{
        fn new(calculation:T)->Cacher<T>{
            Cacher{
                calculation,
                value:None,
            }
        }
        fn value(&mut self,arg:u32)->u32{
            match self.value{
                Some(v)=>v,
                None=>{
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

fn main(){
    let mut c=Cacher::new(|x| x+1);
    let v1 = c.value(1);
    println!("v1={}",v1);
}
```

闭包可以通过三种方式捕获参数的方式，分别是获得所有权、可变借用、不可变借用。这三种捕获值的方式被编码为如下三个Fn trait:
*(1)FnOne消费从环境作用域捕获的变量*

*(2)FnMut获取可变借用值，所以可以改变其环境。*

*(3)Fn从环境不可变的借用值*

示例：
```
fn main(){
let x= vec![1,2,3];
let equal_to_x=move |z| z==x;
let y= vec![1,2,3];
assert!(equal_to_x(y))
}
```
---
## 7月8日

在做rustlings的习题的时候发现我对智能指针的理解还不够深入，于是我又看了看了一下令狐壹冲关于智能指针的讲解。

智能指针是一类数据结构，他们表现类似于指针，但是也拥有额外的元数据，最明显的，他们拥有一个引用计数。引用计数记录智能指针总共有多少个所有者，并且当且没有任何所有者时清除数据。

普通引用和智能指针的一个区别是：应用知识借用数据指针，而智能指针则是拥有指向的数据。

智能指针实现了Deref和Drop trait

几个标准库中的智能指针
*Box<T>,用于堆上分配；*

*Rc<T>,一个引用类型，其数据有多个所有者*

*Ref<T>和RefMut<T>,通过RefCell<T访问>，一个在运行时而不再编译时执行借用规则的类型*

实现Deref trait允许我们重载解引用运算符。

另外添加了leetcode上面的一题，并用C++解答，明天用rust实现