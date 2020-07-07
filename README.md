# DailySchedule
OS Tutorial Summer of Code 2020 by 罗子健
---
## 7月4日
阅读了rust权威指南《The Rust programming Language》 第十章---泛型、trait于生命周期。
完成rustlings上[variable](https://github.com/kszlzj/DailySchedule/tree/master/step0/variables) [primitive_types](https://github.com/kszlzj/DailySchedule/tree/master/step0/primitive_types) [function](https://github.com/kszlzj/DailySchedule/tree/master/step0/functions) [if](https://github.com/kszlzj/DailySchedule/tree/master/step0/if) [test1](https://github.com/kszlzj/DailySchedule/blob/master/step0/test1.rs)章节
---
## 7月5日
将进度推进到了error1.rs
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
---