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

---
## 7月9日

今天把rustlings做完了,还继续复习了智能指针。最重要的是我开始学习不安全的rust。

不安全的rust存在的两大原因：

*静态分析是保守的*

*底层计算机硬件固有的不安全性*

Rust会通过unsafe关键字切换到不安全的Rust

通过unsafe我还学会了C语言与Rust互相调用

我还学会了访问和修改静态变量。
---
## 7月10日

今天复习了迭代器的相关知识。

完成了两道leetcode题目

在做题的过程中发现，Rust在实现数据结构的时候空指针指针用Option代替，这对平时用C++刷题的我来说十分不适应!于是我又回去复习了一下Option的内容。

另外今天把之后rcore的实验环境搭好了，并第一次运行！

---
## 7月11日

今天出去玩了，晚上回来用rust刷了四题leetcode，顺便看了一下RUST标准库里的容器用法

---
## 7月12日

今天是值得纪念的一天，今天我把lab0实现了，并第一次跑起来最小微内核。***是的，这只是一个开头，但里面的乐趣足以让我奋斗终生***

---
## 7月13日

把学堂在线上中断，系统调用这一章看了，并且学习了risc-v架构的汇编语言写法，为lab1做准备

---
## 7月14日
今天有点事，刚写完lab 1，能运行起来，程序可以正常产生中断，明天写实验文档

---
## 7月17日
lab1有一些问题，我重新调整了一下定时器。
在学堂在线上看完物理内存管理：连续地址分配一节

---
## 7月18日
昨天的内容理解不深，我又重新看了一遍，并且阅读了《现代操作系统的相关章节》
仔细阅读了这个[博文](https://blog.csdn.net/ibless/article/details/81367700)

---
## 7月20日
今天在学堂在线上把物理内存管理：非连续内存分配看了。
逐步完善lab2

---
## 7月21日
今天看第六章：虚拟存储概念
完成lab2
lab3做了一部分