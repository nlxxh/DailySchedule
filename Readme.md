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
目前来看，基本的语法和C语言相差不大，也都比较好理解，值得注意的是Rust语言基于表达式的特征，第四章所有权是Rust语言的重点，还没有完全理解，进行了一部分，还没有学完，明天继续。我一直有手写笔记的习惯，学新东西也还是愿意在本子上写写画画一些重点。  

### 事件2：
虚拟机上安装Rust，小练习题完成下载到虚拟机上，家里的网有些慢，常报timeout的错误，所以安装花了比较久的时间。通过命令行实现了基本的Hello,World和Hello,Cargo的程序，知道Rust语言编译运行的基本流程

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
其中第十二章：一个 I/O 项目：构建命令行程序与《Rust 编程之道》第十章的示例代码相关，因此也进行了一部分《Rust 编程之道》第十章的示例代码的学习。但是始终觉得对引用、copy trait那里理解的不太透彻，查了一些资料，也还没有完全弄明白，主要是引用位于函数返回值或参数时没有找到通用的规则，明天继续研究一下

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
练习题的代码和README提交在[DailySchedule/exercises](https://github.com/nlxxh/DailySchedule/tree/master/exercises)一栏，一开始提交到GitHub时遇到了报错failed to push some refs to git，后来通过[教程](https://www.cnblogs.com/yiduobaozhiblog1/p/9125465.html)成功解决，完成全部练习的截图如下![image](https://github.com/nlxxh/picture/blob/master/368526ec23cab14eb405e26f10a5059.png)

### 事件3：
DailySchedule的格式修改的好看了一点


### 预计计划：  
继续学习《Rust 编程之道》第十章的示例代码；开始用Rust语言编中小型程序；解决引用、copy trait的问题；复盘一下小练习题



