## Lab1学习报告

### 基本概念

#### 中断的种类
* 异常（Exception）：执行指令时产生的，通常无法预料的错误。例如：访问无效内存地址、执行非法指令（除以零）等
* 陷阱（Trap）：陷阱是一系列强行导致中断的指令，例如：系统调用（Syscall）等
* 硬件中断（Hardware Interrupt）：CPU 之外的硬件产生的异步中断，例如：时钟中断、外设发来数据等

|中断                 |源头                  |响应方式              |处理机制             |
|---------------------|---------------------|---------------------|---------------------|
|异常                 |应用程序意想不到的行为 |同步                  |杀死应用程序或者重新执行 |
|陷阱                 |应用程序请求OS服务     |同步或异步            |等待和持续          |
|硬件中断             | 外设                 |异步                  |持续，对应用程序透明 |

#### 中断相关寄存器(CSR)

##### 硬件自动填写的寄存器
* sepc：记录触发中断的那条指令的地址
* scause：记录中断发生的原因，还会记录该中断是不是一个外部中断
* stval：记录一些中断处理所需要的辅助信息，比如取指、访存、缺页异常
#####  设置的寄存器
* stvec，设置如何寻找 S 态中断处理程序的起始地址，保存了中断向量表基址 BASE，同时还有模式 MODE
当MODE=0时，设置为 Direct 模式，无论中断因何发生我们都直接跳转到基址pc←BASE   
当MODE=1时，设置为 Vectored 模式，遇到中断我们会进行跳转如下：pc←BASE+4×cause   
只需将各中断处理程序放在正确的位置，并设置好 stvec ，遇到中断的时候硬件根据中断原因就会自动跳转到对应的中断处理程序
* sstatus：S 态控制状态寄存器。保存全局中断使能标志，以及许多其他的状态。可设置此寄存器来中断使能与否
* sie：用来控制具体类型中断的使能，例如其中的 STIE 控制时钟中断使能   
* sip：和 sie 相对应，记录每种中断是否被触发   
仅当 sie 和 sip 的对应位都为 1 时，意味着开中断且已发生中断，这时中断最终触发

#### 中断相关指令

##### 进入和退出中断
* ecall：触发中断，进入更高一层的中断处理流程之中。用户态->内核态中断处理流程，内核态->机器态
* sret：从内核态返回用户态，同时将 pc 的值设置为 sepc（如果需要返回到 sepc 后一条指令，就需要在 sret 之前修改 sepc 的值）
* ebreak：触发一个断点
* mret：从机器态返回内核态，同时将 pc 的值设置为 mepc
##### 操作CSR
* csrrw dst, csr, src：同时读写的原子操作，将指定 CSR 的值写入 dst，同时将 src 的值写入 CSR
* csrr dst, csr：将指定 CSR 的值写入 dst
* csrw csr, src：将指定 src 的值写入 CSR


#### 建立中断机制
* 建立中断服务例程
* 让CPU能响应中断
* 响应并处理中断
* 保存和恢复现场

### 操作流程
    
* 触发中断导致中断发生时，硬件帮我们设置中断原因、中断地址，随后就根据 `stvec`直接跳转到中断处理程序
* 中断处理程序：`SAVE_ALL`保存现场->跳转到中断服务例程->`RESTORE_ALL`恢复现场 

#### 保存和恢复现场
* Context：中断时保存了各种寄存器的结构体，表示原来程序正在执行所在的上下文
```
#[repr(C)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}
```
中断处理程序`_interrupt`：状态的保存，用栈上的一小段空间来把需要保存的全部通用寄存器和 CSR 寄存器(即上下文Context)保存在栈上，保存完之后通过`jal`跳转到 Rust 编写的中断服务例程`handle_interrupt`；而对于恢复，则直接把备份在栈上的内容写回寄存器，和保存是相反的操作，由于涉及到了寄存器级别的操作，需要用汇编来实现，最终通过`sret`返回到sepc指向的地址，即回到触发中断的那条指令所在地址，具体代码不再细述

#### 让CPU能响应中断
* 把中断入口写入 `stvec` 中  
`stvec::write(__interrupt as usize, stvec::TrapMode::Direct);`
* 开启中断使能(时钟中断)
```
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer(); 
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        sstatus::set_sie();
    }
    // 值得注意的是，在开启时钟中断的过程中还调用了设置下一次时钟中断的函数，这里是起预处理的作用，预约第一次时钟中断
    set_next_timeout();
}
```
#### 建立中断服务例程
* 时钟中断
```
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}
//这里值得注意的是，操作系统请求（sbi_call 调用 ecall 指令）SBI 服务来完成时钟中断的设置，OpenSBI 固件在接到 SBI 服务请求后，会帮助 OS 设置下一次要触发时钟中断的时间
pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0);
}
```
* 断点中断
```
//这里值得注意的是，其中 `sepc` 增加 2 字节，改变中断返回地址防止死循环
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}
```
### 总结

* 这里列出了我个人在实现Lab0时遇到的重点、难点，详细的代码不再赘述
* 一开始对代码的大框架是可以理解的，但是细节有一些不太懂的地方，后来通过查资料、实践得以解决，具体是：
  * 模块的互相调用方面：代码的结构是main.rs和interrupt文件夹平级，interrupt文件夹里面有mod.rs、timer.rs等文件，mod.rs文件定义了nit函数，但是main.rs直接声明了`mod interrupt;`并且调用了`interrupt::init();`，开始就不太理解，以为声明的`mod interrupt;`只能是interrupt.rs，后来通过[Rust by Example](https://doc.rust-lang.org/rust-by-example/mod/split.html)知道了这种声明不仅代表interrupt.rs，还可以是interrupt/mod.rs，问题解决
  * 代码的运行流程方面：开始不太懂代码运行的先后顺序，想用gdb调试一下，但是不知道哪里出了问题，gdb设置断点以后continue一运行就退出了，无法调试，百度以后也无法解决，就放弃了，后来在程序中人为加了几个print输出，得到了基本的运行流程，main.rs的rust_main函数是程序的入口函数，先执行`interrupt::init();`进入mod.rs的init函数，执行`handler::init();`这里把中断处理程序的入口设置好，等待中断的触发，然后是执行`timer::init();`这里相当于已经触发了时钟中断，但是因为时钟中断的时间间隔比较长，所以还没有输出，然后是执行一句print输出"mod interrupt initialized"，此时mod.rs的init函数执行完毕，回到main.rs的rust_main函数，`ebreak`触发了断点中断，执行中断处理程序，在中断服务例程中print输出"Breakpoint at ..."，因为对断点中断执行了`context.sepc += 2;`因此只执行一次，一定的时间之后，时钟中断产生了相应的输出"100 tick 200 tick..."，因为时钟中断的`sepc`不变，就会一次次的执行中断处理程序，形成死循环
* 第一次实现一个完整的Lab，虽然花了比较久的时间，但是还是有很多的收获，再熟悉一下这些流程之后，应该会考虑自己实现一个类似的操作 
