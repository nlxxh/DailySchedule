# DailySchedule

## **TOC**

*七月*

| Mon                  | Tues                 | Wed                  | Thur                 | Fri                  | Sat                  | Sun                  |
|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|
|                      |                      | 1                    | 2                    | [3](#0)              | 4  <br> ([D1](#1))   | 5  <br> ([D2](#2))   |
| 6  <br> ([D3](#3))   | 7  <br> ([D4](#4))   | 8  <br> ([D5](#5))   | 9  <br> ([D6](#6))   | 10 <br> ([D7](#7))   | 11 <br> ([D8](#8))   | 12 <br> ([D9](#9))   |
| 13 <br> ([D10](#10)) | 14 <br> ([D11](#11)) | 15 <br> ([D12](#12)) | 16 <br> ([D13](#13)) | 17 <br> ([D14](#14)) | 18 <br> ([D15](#15)) | 19 <br> ([D16](#16)) |
| 20 <br> ([D17](#17)) | 21 <br> ([D18](#18)) | 22 <br> ([D19](#19)) | 23 <br> ([D20](#20)) | 24 <br> ([D21](#21)) | 25 <br> ([D22](#22)) | 26 <br> ([D23](#23)) |
| 27 <br> ([D24](#24)) | 28 <br> ([D25](#25)) | 29 <br> ([D26](#26)) | 30 <br> ([D27](#27)) | 31 <br> ([D28](#28)) |                      |                      |


## OLD TOC Day 0~28


* [Day 0](#0)  
* [Day   1    (2020-07-4)](#Day001)   
* [Day   2    (2020-07-5)](#Day002)   
* [Day   3    (2020-07-6)](#Day003)  
* [Day   4    (2020-07-7)](#Day004)  
* [Day   5    (2020-07-8)](#Day005)  
* [Day   6    (2020-07-9)](#Day006)  
* [Day   7    (2020-07-10)](#Day007)  
* [Day   8    (2020-07-11)](#Day008)  
* [Day   9    (2020-07-12)](#Day009)  
* [Day  10    (2020-07-13)](#Day010)  
* [Day  11    (2020-07-14)](#Day011)  
* [Day  12    (2020-07-15)](#Day012)  
* [Day  13    (2020-07-16)](#Day013)  
* [Day  14    (2020-07-17)](#Day014)   
* [Day  15    (2020-07-18)](#Day015)  
* [Day  16    (2020-07-19)](#Day016)  
* [Day  17    (2020-07-20)](#Day017)  
* [Day  18    (2020-07-21)](#Day018)  
* [Day  19    (2020-07-22)](#Day019)  
* [Day  20    (2020-07-23)](#Day020)  
* [Day  21    (2020-07-24)](#Day021)  
* [Day  22    (2020-07-25)](#Day022)  
* [Day  23    (2020-07-26)](#Day023)  
* [Day  24    (2020-07-27)](#Day024)  
* [Day  25    (2020-07-28)](#Day025)
* [Day  26    (2020-07-29)](#Day026)
* [Day  27    (2020-07-30)](#Day027)  
* [Day  28    (2020-07-31)](#Day028)  

<span id="0"></span>
## Day 0

### 事件1：
开始学操作系统(RISC-V)清华在线课程,2020春季课程，购买操作系统概念的书

### 事件2：
阅读https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code 的文档，先全面了解第一阶段实习的任务

### 事件3：
电脑的VMvare虚拟机一直上不了网，卸载重装的时候出现了问题，电脑开不了机了，重置系统，重新安装软件，心态有点崩，好在已经把基本的软件下回来了，而且虚拟机重装后可以上网了

### 预计计划：  
要开始着手学习Rust语言

<span id="Day001"></span>
## Day 1

### 事件1：
开始学习Rust语言，阅读[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 入门指南
* 猜猜看游戏教程
* 常见编程概念
* 认识所有权   
目前来看，基本的语法和C语言相差不大，也都比较好理解，值得注意的是Rust语言基于表达式的特征，第四章所有权是Rust语言的重点，还没有完全理解，进行了一部分，还没有学完，明天继续。我一直有手写笔记的习惯，学新东西也还是愿意在本子上写写画画一些重点  

### 事件2：
虚拟机上安装Rust，小练习题完成下载到虚拟机上，家里的网有些慢，常报timeout的错误，所以安装花了比较久的时间。通过命令行实现了基本的Hello,World和Hello,Cargo的程序，知道了Rust语言编译运行的基本流程

### 事件3：
继续安装电脑重置后消失的软件

### 预计计划：  
继续看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)；开始解决小练习题

<span id="Day002"></span>
## Day 2

### 事件1：
虚拟机上安装Rust语言编译器，可能因为我的虚拟机版本过低，开始安装intellij和Vscode都没有成功，后来按照[教程](https://blog.csdn.net/Among12345/article/details/81874117)通过Ubuntu make安装成功，在Vscode平台完成了Rust语言环境配置，并且运行了几个小程序，但是总的来说安装和配置环境不太顺利，浪费了比较久的时间

### 事件2：
继续看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 认识所有权
* 使用结构体来组织相关联的数据   
因为事件1花了大部分时间，导致进度有点慢

### 事件3：
编写了一部分小练习题，巩固了一下Rust的语法规范，完成到
* variables
* if
* functions
* test1
* primitive_types
* structs

### 预计计划：  
继续看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)；继续解决小练习题

<span id="Day003"></span>
## Day 3

### 事件1：
今天有点杂事，学习时间比较短，继续看了[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 枚举与模式匹配
* 使用包、Crate和模块管理不断增长的项目    
其中match具有绑定值的模式，可以从枚举成员中提取值的特点很巧妙，但是刚刚接触不太适应，还要继续熟练

### 事件2：
继续解决小练习题，完成到
* strings
* test2
* enums   
其中涉及到match绑定元组、字符串的测试，认识到模式中的变量数必须和待匹配的变量数一致，丰富了教程里绑定枚举的例子

### 预计计划：  
今天学习时间比较少，要追赶一下进度，尽量往后看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)；继续解决小练习题

<span id="Day004"></span>
## Day 4

### 事件1：
继续学了[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 常见集合
* 错误处理
* 泛型、trait 与生命周期
* 测试    
因为练习题遇到，还额外看了宏，感觉这几章的任务量都有点大，而且泛型和trait的概念也不太好理解，所以进度不太快

### 事件2：
继续解决小练习题，完成到
* tests
* modules
* macros
* test4    
今天看书的时间太久了，导致练习题没能如期做完，明天应该可以做完了

### 预计计划：  
继续看Rust 程序设计语言 简体中文版；解决完小练习题；开始用Rust语言编中小型程序
<span id="Day005"></span>
## Day 5

### 事件1：
继续学习[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 一个 I/O 项目：构建命令行程序
* Rust 中的函数式语言功能：迭代器与闭包   
其中第十二章：一个 I/O 项目：构建命令行程序与《Rust 编程之道》第十章的示例代码相关，因此也进行了一部分《Rust 编程之道》第十章的示例代码的学习。但是始终觉得对引用、copy trait那里理解的不太透彻，查了一些资料，也还没有完全弄明白，主要是引用位于函数返回值或参数时没有找到通用的规则，还要继续研究一下

### 事件2：
继续解决小练习题，完成到
* move_semantics
* error_handling     
还有尚未看完的语法，小练习也还没做完，拖了好几天了，立flag，明天一定要做完了

### 事件3：
学习了一部分《Rust 编程之道》第十章的示例代码


### 预计计划：  
继续看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)；解决完小练习题；开始用Rust语言编中小型程序；解决引用、copy trait的问题；改一下DailySchedule的格式
<span id="Day006"></span>
## Day 6

### 事件1：
继续学习[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* 智能指针前四节
* 无畏并发     
线程、共享状态都是很新的概念，以前从来没有见到过，现在感觉理解还是比较浅显，随着学习的深入应该会有更深的理解

### 事件2：
终于解决了所有的小练习题，今天完成了
* option
* clippy     
* standard_library_types
* traits
* generics
* threads
* conversations    
练习题的代码和README提交在[DailySchedule/exercises](https://github.com/nlxxh/DailySchedule/tree/master/exercises)一栏，一开始提交到GitHub时遇到了报错failed to push some refs to git，后来通过[教程](https://www.cnblogs.com/yiduobaozhiblog1/p/9125465.html)成功解决，完成全部练习的截图如下
![image](https://github.com/nlxxh/picture/blob/master/368526ec23cab14eb405e26f10a5059.png)

### 事件3：
DailySchedule的格式修改的好看了一点


### 预计计划：  
继续学习《Rust 编程之道》第十章的示例代码；开始用Rust语言编中小型程序；解决引用、copy trait的问题；复盘一下小练习题
<span id="Day007"></span>
## Day 7

### 事件1：
继续学习[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，完成到
* Rust 的面向对象编程特性

### 事件2：
完成了编写5道中小型练习题，Rust版本和原Python或C语言版本已打包在一起上传
* Learn C The Hard Way 中文版的[ex10.c](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-c/ex15)、[ex16.c](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-c/ex16)
* Learn-Python-3-the-Hard-Way的[ex18.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex18)、[ex35.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex35)、[ex40.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex40)   
感觉LeetCode上面的题目以算法为主，不太适合用现在学的Rust来编程，所以就打算使用笨方法系列了，但是[Learn-Python-3-the-Hard-Way](https://github.com/cnR1ce/Learn-Python-3-the-Hard-Way/tree/master/codes)上面的题目有的太简单了，[Learn C The Hard Way 中文版](https://docs.kilvn.com/lcthw-zh/)的题目又有很多大型的数据结构的实现，要精挑细选15道左右还比较困难

### 事件3：
自己编了几个程序，初步搞懂了copy trait;又复习了一下之前做的比较坎坷的小练习题[DailySchedule/exercises](https://github.com/nlxxh/DailySchedule/tree/master/exercises)，有不少收获
```
//DailySchedule/exercises/enums/enums3.rs
enum Message {
    ChangeColor(u8, u8, u8),
    ...
}
fn process(&mut self, message: Message) {
        match message{
              Message::ChangeColor(r, g, b)  => {self.change_color((r,g,b));},//认识到match绑定要求模式中的变量数要和待匹配的变量数一致，即匹配的是一个元组中的三个变量，而不是一个元组
              ...
} }
```
```
//DailySchedule/exercises/error_handling/errorsn.rs
fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let num: i64 = line.trim().parse()?;//认识到?运算符在Result<_,_>是Ok时，返回的是Ok中的值，而不是Ok(_)
    ...
}
```
```
//DailySchedule/exercises/traits/traits2.rs
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self{
       let mut v=self;
       v.push("Bar".to_string());//这个做了比较久，一开始就是写self.push("Bar".to_string());提示返回值类型不匹配才明白   
       v
} }
```
还有就是threads、arc的部分觉得比较难，而且不太熟练，在做实验中遇到再继续巩固，关于迭代器的方法看标准库有点慢，感觉这个[教程](https://blog.csdn.net/guiqulaxi920/article/details/78823541?utm_medium=distribute.pc_relevant_t0.none-task-blog-BlogCommendFromMachineLearnPai2-1.compare&depth_1-utm_source=distribute.pc_relevant_t0.none-task-blog-BlogCommendFromMachineLearnPai2-1.compare)还讲的比较细致

### 事件4：
完成了《Rust 编程之道》第十章的示例代码，上传到了[DailySchedule/csv_challenge](https://github.com/nlxxh/DailySchedule/tree/master/csv_challenge)，根据这个示例代码对Rust的模块化编程有了更深入的了解。另外，按照书上的代码我开始不能正确运行，会提示main error:Program("column name doesn't exist in the input file")，发现是core/write.rs的问题，debug发现headers都可以正常输出，那么就是和修改的目标column匹配的问题，应该是输入的参数还要通过trim方法去掉空格等才能和column匹配，经过修改，运行正确
```
//let column_number=columns.iter().position(|&e| e==column);
let column_number=columns.iter().position(|&e| e.trim()==column);
```

### 预计计划：  
开始step1 risc-v系统结构的学习；继续用Rust语言编中小型程序
<span id="Day008"></span>
## Day 8

### 事件1：
基本完成阅读《Rust 编程之道》第三章的内容，因为已经仔细过了一遍[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/title-page.html)，所以看的比较快

### 事件2：
完成了step2 实验的[环境部署](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/env.html)，在虚拟机进行实验，配置环境花了比较多的时间

### 预计计划：  
开始step1 risc-v系统结构的学习；继续用Rust语言编中小型程序
<span id="Day009"></span>
## Day 9

### 事件1：
阅读了一部分《计算机组成与设计（RISC-V版）》，今天事情比较多，学习的时间少，要追赶一下进度

### 事件2：
参加第一次线上交流

### 预计计划：  
继续step1 risc-v系统结构的学习；继续用Rust语言编中小型程序
<span id="Day0010"></span>
## Day 10

### 事件1：
完成阅读《计算机组成与设计（RISC-V版）》的前两章，很多内容都是和计算机系统基础有重合的，看的比较顺利

### 事件2：
学习了一部分[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)的第十章内容

### 事件3：
观看了[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)前三章的内容，刚开始入门操作系统的内容，进展不是很快，结合[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)，初步搞懂了中断的处理机制，还需要实验检验，计划实验和视频结合的形式学习，看完一部分的视频就去进行对应的实验操作，以加深对知识的印象

### 事件4：
完成阅读《Rust 编程之道》第四章的内容

### 事件5：
按照step2 实验的[gdb调试方法](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/gdb.html)安装了riscv64-unknown-elf-gdb

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)；开始实现Lab0的内容
<span id="Day0011"></span>
## Day 11

### 事件1：
完成了编写1道中小型练习题
* Learn-Python-3-the-Hard-Way的[ex33.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex33)

### 事件2：
按照[Lab0 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html)实现了Lab0的内容，得到了正确的输出，并且按照[GDB调试方法](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/gdb.html)实现了调试的界面，手动使用 GDB 对 rCore 进行 debug，一开始的时候不理解让 QEMU在 1234 端口等待调试器接入的含义，后来才明白是开两个终端，一个运行QEMU，另一个运行GDB，GDB连接QEMU来进行调试，但是对Lab0的代码不是完全理解，还要继续看一下相关的知识点

### 事件3：
大致完成阅读《Rust 编程之道》第十三章的内容

### 事件4：
实现Lab0的过程中，结合阅读了部分[《使用 Rust 编写操作系统》](https://github.com/rustcc/writing-an-os-in-rust)

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；实现Lab1
<span id="Day0012"></span>
## Day 12

### 事件1：
完成了编写2道中小型练习题
* Learn C The Hard Way 中文版的[ex15.c](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-c/ex15)
* Learn-Python-3-the-Hard-Way的[ex39.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex39)

### 事件2：
又重新按照[Lab0 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html)捋了一遍Lab0的流程，回看了部分[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第二章的内容，基本理解了Lab0每一步的目的和代码

### 事件3：
按照[Lab1 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-1/guide/intro.html)完成了Lab1时钟中断之前的内容，发现了几处教程的错误
* os/src/interrupt/handler.rs
```
//global_asm!(include_str!("./interrupt.asm"));
global_asm!(include_str!("interrupt.asm"));
```
```
//use riscv::register::stvec;
use riscv::register::{stvec,scause::Scause};
```
* 输出结果
```
/*Hello rCore-Tutorial!
mod interrupt initialized
panic: 'Interrupted: Exception(Breakpoint)'*/
mod interrupt initialized
panic: 'Interrupted: Exception(Breakpoint)'
```

### 预计计划：  
继续用Rust语言编中小型程序；继续实现并理解Lab1
<span id="Day0013"></span>
## Day 13

### 事件1：
完成了编写2道中小型练习题
* Learn C The Hard Way 中文版的[ex14.c](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-c/ex14)
* Learn-Python-3-the-Hard-Way的[ex38.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex38)

### 事件2：
按照[Lab1 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-1/guide/intro.html)完成了Lab1时钟中断的内容，发现了几处教程的错误
* os/src/main.rs 其中rust_main函数去掉unreachable!()之后，还要将返回类型的！去掉
```
//pub extern "C" fn rust_main() -> ! {}
pub extern "C" fn rust_main()  {}
```
* os/src/interrupt/handler.rs
```
//use riscv::register::{stvec,scause::Scause};
use riscv::register::{stvec,scause::{Scause,Trap,Exception,Interrupt}};
```
```
use super::timer;
```
```
/*panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );*/
/* panic!(
        "Unresolved interrupt: {:?}\n{:x}\nstval: {:x}",
        scause.cause(),
        context.sepc,
        stval
    );*/
```
### 事件3：
系统学习了一下git的用法，之前本地仓库和远程仓库一直没有很好的实现同步，向GitHub上传文件有时会出问题，修改好了之后，上传文件更加方便，建立了同步的本地仓库，随时更新，随时上传

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；完成Lab1的学习报告
<span id="Day0014"></span>
## Day 14

### 事件1：
完成了编写1道中小型练习题
* Learn C The Hard Way 中文版的[ex13.c](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-c/ex13)

### 事件2：
仔细理解Lab1的实现流程，一些遇到的困难和解决办法会整理在Lab1的学习报告，应该还需要一天才能写完

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；完成Lab1的学习报告
<span id="Day0015"></span>
## Day 15

### 事件1：
整理完成了Lab1的代码和学习报告，提交在[DailySchedule/Lab/Lab1](https://github.com/nlxxh/DailySchedule/tree/master/Lab/Lab1)

### 事件2：
观看了[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第四讲前四节的内容，开始学习了内存分配的知识，今天学习时间不是太多，视频还没有看完，明天继续

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)
<span id="Day0016"></span>
## Day 16

### 事件1：
参加了第二次线上交流，和大家做了分享，时间比较仓促，没来得及好好准备什么，但是看大家的进度，感觉自己稍微有点落后，要加快速度呀

### 事件2：
观看完[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第四讲和第五讲的内容，学习了物理内存管理的连续内存分配和非连续内存分配，比较容易理解，还是要结合实验去巩固这些知识，明天开始做实验，视频上还提出了[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)的第十章内容比较有帮助，明天也会辅助看一下

### 预计计划：  
继续学习[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)；继续用Rust语言编中小型程序；开始学习Lab2
<span id="Day0017"></span>
## Day 17

### 事件1：
基本阅读完[RISC-V手册：一本开源指令集的指南](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)第十章的内容，后面虚拟内存的部分和[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第五讲视频的内容有很多重合，所以只是大致过了一遍

### 事件2：
按照[Lab2 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-2/guide/intro.html)实现Lab2的内容，发现代码有很多缺失，在[rCore-Tutorial仓库](https://github.com/rcore-os/rCore-Tutorial/tree/master/os)里面找到了完整的代码，实现了Lab2，得到了正确的运行结果，但是感觉代码量比Lab1大了很多，还没能好好理解

### 事件3：
试图用Rust编一下双向链表，但是我尝试用Box和Rc都各种报错，borrowed、mutable、copy trait等等，改了太久，花了太多时间了，就战略性放弃了，打算再选几个[Learn-Python-3-the-Hard-Way](https://github.com/cnR1ce/Learn-Python-3-the-Hard-Way/tree/master/codes)的练习题做完15道，然后专心看Lab

### 预计计划：  
整理Lab2的学习报告；用Rust语言编中小型程序结束
<span id="Day0018"></span>
## Day 18

### 事件1：
完成了编写4道中小型练习题
* Learn-Python-3-the-Hard-Way的[ex32.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex32)
* Learn-Python-3-the-Hard-Way的[ex17.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex17)
* Learn-Python-3-the-Hard-Way的[ex21.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex21)
* Learn-Python-3-the-Hard-Way的[ex26.py](https://github.com/nlxxh/DailySchedule/tree/master/rust-code/rust-python/ex26)
已经完成了15道中小型练习题的编写，都上传在[DailySchedule/rust-code](https://github.com/nlxxh/DailySchedule/tree/master/rust-code)，编写完整的代码以后，对rust的语法更加熟悉，对Option、match、Result等rust的特有语法也掌握的比较熟练了，但是为了节省时间，没有成功编写复杂的数据结构的代码，总是遇到所有权方面的报错，当时编双向链表的时候就是感觉怎么写都能巧妙的遇上error，也就先放弃了，毕竟还是要以Lab的实现为主

### 事件2：
整理完成了Lab2的代码和学习报告，提交在[DailySchedule/Lab/Lab2](https://github.com/nlxxh/DailySchedule/tree/master/Lab/Lab2)

### 预计计划：  
开始实现Lab3；继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)
<span id="Day0019"></span>
## Day 19

### 事件1：
实现部分Lab3，同步写学习报告，内容比较多，应该还要一天才能完成，但是虽然知识点多，但是感觉代码和知识点的对应比较好，理解起来比较容易

### 事件2：
学习了[writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust)的内存分页简介相关内容

### 预计计划：  
完成Lab3的代码和学习报告；继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)
<span id="Day0020"></span>
## Day 20

### 事件1：
整理完成了Lab3的代码和学习报告，提交在[DailySchedule/Lab/Lab3](https://github.com/nlxxh/DailySchedule/tree/master/Lab/Lab3)

### 事件2：
学习了[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第九讲的部分内容，为Lab4做准备，还没有学完，明天继续

### 预计计划：  
继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)；开始实现Lab4
<span id="Day0021"></span>
## Day 21

### 事件1：
按照[Lab4 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-4/guide/intro.html)开始实现Lab4

### 事件2：
学习了[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)第九讲和第十讲的部分内容，这几天学校来这边招生，会去帮一下忙，进度可能要拖慢了

### 预计计划：  
继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)；继续实现Lab4
<span id="Day0022"></span>
## Day 22

### 事件1：
按照[Lab4 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-4/guide/intro.html)继续实现Lab4

### 预计计划：  
继续观看[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)；继续实现Lab4
