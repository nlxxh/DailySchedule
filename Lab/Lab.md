# Lab学习报告

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

## Lab2学习报告

### 动态内存分配

#### Buddy System

* 算法介绍：一种连续内存分配算法，每次分配的时候都恰好分配一块大小是`2`的幂次的内存，且要保证内存的开头地址需要是对齐的，也就是内存的开头地址需要是这块内存大小的倍数
* 分配内存：寻找大小合适的内存块(大于等于所需大小并且最接近2的幂)
  * 如果找到了，分配给应用程序
  * 如果没找到，分出合适的内存块
    * 对半分离出高于所需大小的空闲内存块
    * 如果分到最低限度，分配这个大小
    * 重复该步骤直到一个合适的块
* 实现思路：线段树(在每个线段树节点上存当前区间上所能够分配的最大`2`的幂次的内存大小`m`)`
  * 找到合适的内存块：为了尽可能满足分配的对齐需求，先尝试右子树，再尝试左子树，直到找到一个节点满足这个区间够分配，且它的左右子区间都不够分配，就将这个区间整体分配出去，将当前区间的`m`值改为`0`
  * 标注可选的内存块：左右子区间有已经分配出去的，自下而上进行`m`值的更新，`pa.m<-max(ls.m,rs.m)`
  * 回收时只需找到分配时的那个节点，将其`m`值改回去，同时同样自下而上进行`m`值更新即可
* 本实验直接使用开发好的 `buddy system allocator`
```
//这段空间编译后会被放在操作系统执行程序的 bss ，因为这段是一个静态的没有初始化的数组，作为提供给动态分配器的内存
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
```
```
//堆的动态内存分配器
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();
```
```
// 告诉分配器使用这一段预留的空间作为堆
unsafe {
    HEAP.lock().init(
        HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
    )
}
```
### 物理内存探测

* 默认的 DRAM 物理内存地址范围就是 `[0x80000000,0x88000000)`
  * 物理地址空间 `[0x80000000,0x80200000)` 被 OpenSBI 占用
  * 物理地址空间 `[0x80200000,KernelEnd)` 被内核各代码与数据段占用
  * 可以用来分配内存的物理地址范围是：`[KernelEnd, 0x88000000)`
    * `KernelEnd`为内核代码结尾的物理地址在，由 `linker.ld` 指定
```
lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}
extern "C" {
    /// 由 `linker.ld` 指定的内核代码结束位置
    /// 作为变量存在 [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}
```
### 物理内存管理

#### 物理页
* 通常，我们在分配物理内存时并不是以字节为单位，而是以一物理页(Frame)，即连续的 `4 KB` 字节为单位分配。我们希望用物理页号(PPN)来代表一物理页，实际上代表物理地址范围在`[PPN×4KB,(PPN+1)×4KB)`的一物理页
* 物理页号与物理页形成一一映射，为了能够使用物理页号这种表达方式，每个物理页的开头地址必须是 `4 KB`的倍数
#### 分配和回收
(这里教程的代码不太全，按照rCore-Tutorial完善了完整的代码，这部分的代码有点多，就先对代码的大框逻辑做了理解，一些实现的细节先忽略)
* `FrameTracker`：在内存中划一片连续区域，作为一个物理帧的标识
```
//FrameTracker结构体的成员PhysicalPageNumber在memory/address.rs定义，代表物理页号，实现了`implement_address_to_page_number`宏，实现了From Trait，  
//也自动实现了Into Trait，即通过物理页号与页的大小(`PAGE_SIZE`)相乘，可以得到物理地址，完成了页号与地址的转换
pub struct FrameTracker(pub(super) PhysicalPageNumber);
impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}
```
* `Allocator`： trait 封装起来的物理页分配器
```
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}
```
```
// FrameAllocator对Allocator trait 实例化，要求成员分配器allocator实现了Allocator trait，在algorithm/src/allocator提供的StackedAllocator和SegmentTreeAllocator结构体分别实现了
//Allocator trait的链表和线段树算法，具体代码不再详述，所以FrameAllocator通过指定T为StackedAllocator或SegmentTreeAllocator可以得到分配器的new、alloc、dealloc功能
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}
//这里给出alloc方法的代码，可见alloc方法返回的是MemoryResult<FrameTracker>，和前述的FrameTracker结构体构建了联系，即分配成功时，可以得到一个物理帧
impl<T: Allocator> FrameAllocator<T> {
  /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }   
    //省略new和dealloc方法
}
```
 ```
//这个分配器会以一个 PhysicalPageNumber 的 Range 初始化，然后把起始地址记录下来(FrameAllocator.start_ppn)，把整个区间的长度告诉具体的分配器算法(这里AllocatorImpl默认是StackedAllocator)，
//当分配的时候就从算法中取得一个可用的位置作为 offset，再加上起始地址返回回去(指alloc方法)，这里的Range结构体定义于memory/range.rs，实现的From方法可以得到区间的初始和结束的位置
//这里使用 spin::Mutex<T> 给这段数据加一把锁，一个线程试图通过 lock() 打开锁来获取内部数据的可变引用，如果钥匙被别的线程所占用，那么这个线程就会一直卡在这里；
//直到那个占用了钥匙的线程对内部数据的访问结束，锁被释放，将钥匙交还出来，被卡住的那个线程拿到了钥匙，就可打开锁获取内部引用，访问内部数据
 lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}
```
### 总结

* 开始觉得Lab2的文件和代码量太多了，有点难以招架，后来就是先按照rCore-Tutorial的代码把正确结果实现，然后再结合教程的介绍一点一点看代码整体的逻辑，基本已经梳理清楚了，整理在这里，但是具体的算法，比如线段树的实现，还没有好好看懂，但是感觉时间有点紧迫，先往下做实验吧，再回头看这些细节的地方

## Lab3学习报告

### 从虚拟内存到物理内存

#### 虚拟地址和物理地址

* 物理地址：物理地址就是内存单元的绝对地址
  * 帧：物理页面，把物理地址空间划分为大小相同的基本分配单位，每个物理页帧大小为4KB
* 虚拟地址：虚拟地址是操作系统给运行在用户态的应用程序看到的假地址
  * 页：虚拟页面，把逻辑(虚拟)地址空间划分为大小相同的基本分配单位，每个虚拟页面大小为4KB
* 转换：在程序中通过虚拟地址假想着自己在访问一块虚拟内存的时候，需要有一种机制，将虚拟地址转化为物理地址，交给 CPU 来根据它到物理内存上进行实打实的访问。而这种将虚拟地址转化为物理地址的机制，在 riscv64 中是通过页表来实现的
* Sv39物理地址：Sv39物理地址有56位
  * `PPN`：物理页号(帧号)，`page offset`：帧内偏移
  * 物理地址=`PPN*4K+page offset`
  
|PPN[2]               |PPN[1]               |PPN[0]               |page offset          |
|---------------------|---------------------|---------------------|---------------------|
|26                   |9                    |9                    |12                   |
* Sv39虚拟地址：Sv39虚拟地址有64位，但是只有低39位有效，规定63-39位的值必须等于第38位的值，否则会认为该虚拟地址不合法，在访问时会产生异常
  * `VPN`：虚拟页号，`page offset`：页内偏移
  * 虚拟地址=`VPN*4K+page offset`

|VPN[2]               |VPN[1]               |VPN[0]               |page offset          |
|---------------------|---------------------|---------------------|---------------------|
|9                    |9                    |9                    |12                   |
* 要实现虚拟地址到物理地址的映射，就是虚拟页到物理帧的映射，而页内偏移`page offset`等于帧内偏移`page offset`，也就是要实现虚拟页号`VPN`到物理帧号`PPN`的映射，这就是页表所做的事情

#### 页表项
* 如果一个虚拟页号通过某种手段找到了一个页表项`PTE`，并通过读取上面的物理帧号完成映射，我们称这个虚拟页号通过该页表项完成映射
* Sv39页表项：Sv39里面的一个页表项大小为64位8字节，其中第53-10共44位为一个物理页号，表示这个虚拟页号映射到的物理页号。后面的第9-0位则描述映射的状态信息

|Reserved             |PPN                  |Flags                |
|---------------------|---------------------|---------------------|
|10                   |44                   |10                   |

#### 多级页表

* 在Sv39中采用三级页表，即将27位的虚拟页号分为三个等长的部分，`VPN[2]`为三级索引，`VPN[1]`为二级索引，`VPN[0]`为一级索引
* 页表也分为三级页表，二级页表，一级页表，页表的大小和一个物理帧的大小相同，所以可以把一个页表放到一个物理页帧中，并用一个物理页号来描述它
  * 三级页表的物理页号为`PPN3`，三级页表的每个页表项中的物理页号`PPN2`可描述一个二级页表；二级页表的每个页表项中的物理页号`PPN1`可描述一个一级页表；一级页表的每个页表项中的物理页号`PPN0`描述一个要映射到的物理页
* 虚拟页号映射到物理帧号的流程：
  * 索引控制虚拟页号范围在`(VPN[2],Any,Any)`的三级页表项，其地址为`PPN3*4K+VPN[2]*8`，从这个页表项里读出二级页表的物理页号`PPN2`
  * 索引控制虚拟页号范围在`(VPN[2],VPN[1],Any)`的二级页表项，其地址为`PPN2*4K+VPN[1]*8`，从这个页表项里读出二级页表的物理页号`PPN1`
  * 索引控制虚拟页号范围在`(VPN[2],VPN[1],VPN[0])`的一级页表项，其地址为`PPN1*4K+VPN[0]*8`，从这个页表项里读出的物理页号`PPN0`就是虚拟页号`(VPN[2],VPN[1],VPN[0])`所要映射到的物理页号
  
#### 页表基址
  
* 页表的基址(起始地址)一般会保存在一个特殊的寄存器中。在 RISC-V 中，这个特殊的寄存器就是页表寄存器 satp
  * MODE: 控制CPU使用哪种页表实现，MODE设置为8即表示CPU使用Sv39 
  * PPN：存放三级页表所在的物理页号`PPN3`。这样，给定一个虚拟页号，CPU 就可以从三级页表开始一步步的将其映射到一个物理页号
  
| MODE                |ASID                 |PPN                  |
|---------------------|---------------------|---------------------|
|4                    |16                   |44                   |

#### 快表TLB

* 快表TLB：记录近期已完成的虚拟页号到物理页号的映射，提高效率
  * 如果修改了`satp`寄存器或者修改了一个页表项，就改变了映射方式，需要进行`sfence.vma`指令刷新整个TLB
  * `sfence.vma`指令不加参数，会刷新整个TLB，如果在后面加上一个虚拟地址，只刷新这个虚拟地址的映射

### 修改内核

#### 把内核代码转移到虚拟地址空间
* linker.ld：将内核代码放在虚拟地址空间中以`0xffffffff80200000`开头的一段高地址空间中，即原来放在`0x80200000`起始地址的全部内核结构被平移到了`0xffffffff80200000`的地址上，映射关系为：虚拟地址减去偏移量`0xffffffff00000000`为原来的物理地址，满足线性映射
```
BASE_ADDRESS = 0xffffffff80200000; /* 内核的基址修改为虚拟地址 */
```
```
. = ALIGN(4K); /* 加入对齐，不同的段放在不同的页上，标注各自特定的属性 */
```
* memory/config.rs：`KERNEL_END_ADDRESS`修改为虚拟地址并加入偏移量
```
lazy_static! {
    ///可以用来分配的内存起始地址修改为虚拟地址
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize); 
}

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;
```
* 当 OpenSBI 启动完成之后：
  * CPU 状态：处于 S Mode ，寄存器 `satp` 的 MODE 字段被设置为 Bare 模式，即无论取指还是访存我们通过物理地址直接访问物理内存
  * 因为 `linker.ld`，代码中 `boot_stack_top` 、`rust_main` 等符号的地址都是虚拟地址（高地址）     
  
* 目前 CPU 将地址都当成物理地址处理，这样，我们跳转到 `rust_main` 就会跳转到 `0xffffffff00000000+` 的一个物理地址，导致问题，所以要修改 `entry.asm`， 恰当构造页表，来对于内核所属的虚拟地址，实现这种虚拟地址到物理地址的映射，这里构造的是最简单的大页，实现 `0xffff_ffff_8000_0000 -> 0x8000_0000`，只需要分配一页内存用来存放三级页表`boot_page_table`

* entry.asm：在进入`rust_main`之前完成一个从物理地址访存模式到虚拟访存模式的转换
```
_start:
    # t0 存储 boot_page_table 的虚拟地址
    lui t0, %hi(boot_page_table) 
    li t1, 0xffffffff00000000
    # t0 存储 boot_page_table 的物理地址
    sub t0, t0, t1
    # t0 存储 boot_page_table 的物理页号，相当于物理地址除以4K
    srli t0, t0, 12
    # 8 << 60 是 satp 中使用 Sv39 模式的记号
    li t1, (8 << 60)
    or t0, t0, t1
    # 写入 satp 并更新 TLB，此时 satp 表示使用 Sv39 模式，且 PPN 存放页表 boot_page_table 的物理页号
    csrw satp, t0
    sfence.vma
    # 已经搭建出了一个虚拟内存空间
```
```
# 启动时的一个简单页表，内核初始映射
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 507 * 8
    # 第 510 项(510 的二进制是要索引虚拟地址的 VPN[2]，注意虚拟地址的第30-38位为VPN[2])：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1，表示 1GB 的一个大页
    .quad (0x80000 << 10) | 0xcf
    .quad 0
```
* 综上，要进入虚拟内存访问方式，需要如下步骤：
  * 分配页表所在内存空间并初始化页表
  * 设置好页基址寄存器（指向页表起始地址）
  * 刷新 TLB

### 实现页表

* 在 riscv crate 和内核实现中，需要为页表机制提供如下支持：
  * 页表项`PageTableEntry`和页表`PageTable`
  
#### 页表项

* 页表项`PageTableEntry`：对一个 usize（8 字节）的封装，按照页表项的定义，划分为物理页号、标志位
  * 涉及到页表项的位级别的操作`set_bits`等，需要引入相应的crate
  * 赋予页表项新建页表项`new`、更新物理页号和Valid位`update_page_number`、清除`clear`、获取物理页号`page_number`、获取物理地址`address`、获取标志位信息`flags`、判断页表项是否为空`is_empty`、判断页表项是否指向下一级页表`has_next_level`、获取下一级页表`get_next_table`的方法
```
/// Sv39 结构的页表项
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);
/// Sv39 页表项中标志位的位置
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
/// Sv39 页表项中物理页号的位置
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;
```

#### 页表

* 单一页表页面`PageTable`：大小为4K(PAGE_SIZE)，包含512条(PAGE_SIZE / 8)页表项`PageTableEntry`
  * 赋予页表清零`zero_init`的方法
* 页表指针`PageTableTracker`：`PageTableTracker`的结构对`FrameTracker`封装。因为页表页面`PageTable`和物理页大小相同，所以直接把物理页当做页表进行读写，而`PageTableTracker`作为页表指针，保存在某个线程的元数据中（也就是在操作系统的堆上），指向一个物理页也就是真正的页表`PageTable`，又因为`PageTableTracker`赋予了自动解引用的特性，所以可以直接把`PageTableTracker`当成 `PageTable`对待
  * 赋予页表指针创建空的页表`new`、获取物理页号`page_number`的方法
```
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}
```
```
pub struct PageTableTracker(pub FrameTracker);
```

### 实现内核重映射

* 已经建立的内核初始映射`boot_page_table`比较粗糙，而一个程序不同的段`.text段`、`.data段`等需要有不同的权限，所以要对这些段进行重映射，即一段内存（可能是很多个虚拟页）以线性的形式映射到很多个物理页上，同时这个内存段将会有一个统一的属性和进一步高层次的管理

#### 内存段

* 内存段`Segment`：一段连续的虚拟页范围，每一页通过线性映射直接偏移到一个物理页(操作系统使用)，或者每个虚拟页调用物理页分配器分配一个物理页(用户使用)
  * 赋予内存段获取虚拟页号区间`page_range`、遍历对应的物理地址`iter_mapped`的方法
    * `iter_mapped`方法：判断映射类型`Segment.map_type`，如果是线性映射，可以直接将虚拟地址转换为物理地址；如果是按帧分配映射，则无法直接获得物理地址，需要分配
```
/// 内存段映射的类型
#[derive(Debug)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    Framed,
}

/// 内存段
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 所映射的虚拟地址
    pub range: Range<VirtualAddress>,
    /// 权限标志
    pub flags: Flags,
}
```

#### 封装页表和内存段
* `Mapping`：对页表和内存段进行组合和封装，借助其中对页表的操作实现对内存段的映射，也是对页表从单级到三级的封装，实现Sv39中采用的三级页表
  * 赋予`Mapping`创建一个有根节点的映射`new`、查找给定虚拟页号的三级页表项`find_entry`、为给定的虚拟页号和物理页号建立映射关系`map_one`、为一段连续的Segment和多个物理页面建立映射关系`map`、激活页表`activate`、移除一段Segment的映射`unmap`、查找虚拟地址对应的物理地址`lookup`的方法
    * `map`方法：判断映射类型`Segment.map_type`，如果是线性映射，直接对虚拟页号进行转换得到物理页号，再通过`map_one`方法建立映射，如果有需要，就拷贝数据；如果是按帧分配映射， 需要通过`FRAME_ALLOCATOR.lock().alloc()`分配物理页面，通过`map_one`方法建立虚拟页号和手动分配的物理页号的映射，拷贝数据需要额外注意区间与整页不对齐的情况，最后要把虚拟页号和分配的物理页面的映射信息保存在`Segment.mapped_pairs`
    * `activate`方法：更新`satp`寄存器，即设置为Sv39模式并且将根页表的页号加载到`PPN`位，同时刷新`TLB`
    * `lookup`方法：通过`satp`寄存器找到根页表，按照给定的虚拟地址的三级页号、二级页号、一级页号，一级一级的查找对应的页表中对应的页表项，直到页表项中的物理页号指向待查找的物理页(支持大页)，用这个物理页号转换成的物理地址与虚拟地址中的偏移量相加，得到此虚拟地址对应的物理地址
  
```
#[derive(Default)]
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}
```
* memory/mapping/mapping.rs: impl Mapping/fn find_entry：感觉这部分比较重要，贴出代码，仔细分析一下
```
pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
    // 从根页表(三级页表)开始向下查询，先把根页表的物理页号转换成物理地址，然后物理地址经过deref_kernel解引用可以转换成&mut PageTable，即root_table是对根页表页面的可变引用
    let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
    // vpn.levels()[0]得到了虚拟地址的三级页号，即虚拟地址定义中的VPN[2]，从而在根页表中找到了对应的页表项，即entry是根页表中对应页表项的可变引用，也可能是不存在的
    let mut entry = &mut root_table.entries[vpn.levels()[0]];
    // 在二级页表和一级页表中继续查询，vpn_slice是虚拟地址的二级页号或一级页号
    for vpn_slice in &vpn.levels()[1..] {
        if entry.is_empty() {
            // 如果页表不存在，则需要分配一个新的页表
            let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
            let new_ppn = new_table.page_number();
            // 将新页表的页号写入当前的页表项
            *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
            // 保存页表
            self.page_tables.push(new_table);
        }
        // 进入下一级页表（使用偏移量来访问物理地址），页表项的get_next_table()方法返回下一级页表的可变引用，同样，在下一级页表中找到对应页表项的可变引用，更新entry
        entry = &mut entry.get_next_table().entries[*vpn_slice];
    }
    // 此时 entry 位于第一级页表
    Ok(entry)
}
```
#### 封装所有内存段和映射关系

* `MemorySet`：把内核的每个段根据不同的属性写入封装的`Mapping`中，然后形成一个新的结构`MemorySet`给线程使用。所以，每个线程都将会拥有一个`MemorySet`，其中存的将会是「它看到的虚拟内存空间分成的内存段」和「这些段中包含的虚拟页到物理页的映射」
  * 赋予`MemorySet`创建内核重映射`new_kernel`、激活页表`activate`、通过elf文件创建内存映射`from_elf`、添加一个Segment的内存映射`add_segment`、移除一个Segment的内存映射`remove_segment`、检测一段内存区域和已有的是否有重叠`overlap_with`的方法
    * `new_kernel`方法：建立每个内存段的`Segment`结构体，存入`MemorySet.segments`，包括`.text 段，r-x`,`.rodata 段，r--`,`.data 段，rw-`,`.bss 段，rw-`,`剩余内存空间，rw-`，同时，将每个字段在页表中进行映射，即对每个字段调用`Mapping.map`方法，存入`MemorySet.mapping`
```
/// 一个进程所有关于内存空间管理的信息
pub struct MemorySet {
    /// 维护页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
}
```
```
let remap = memory::mapping::MemorySet::new_kernel().unwrap();
remap.activate();
/// 此时，所用的所有逻辑已经建立在了新构建的页表上，而不是那个粗糙的 boot_page_table
```

### 总结

* Lab3的知识点比较多，涉及虚拟地址和物理地址的概念和关系；利用页表完成虚拟地址到物理地址的映射；实现内核空间段的重映射，但是因为在[操作系统(RISC-V)](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=learn_title)的视频上学习了一遍、在[writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust)看了一遍、又跟着[Lab3 实验指导--rcore tutorial教程第三版](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-3/guide/intro.html)过了一遍，所以感觉对这些基础知识比较理解了，把知识点和代码的难点基本完整的整理在这里。跟着教程实现，思路一直比较清晰，而且代码和知识点的对应也比较清楚，总的来说，个人感觉比Lab2顺利，但是实现Lab3的过程中，也有回看Lab2.md，知识也有相互的对应，比如页表是建立在物理帧FrameTracker上，对Lab2的理解也有帮助，时间紧迫，就先跳过页面置换，进行Lab4了

## Lab4学习报告

### 线程和进程

#### 基本概念

* 进程：一个具有一定独立功能的程序在一个数据集合上的一次动态执行过程
  * 程序=文件
  * 进程=执行中的程序=程序+执行状态
* 线程：将“正在运行”的动态特性从进程中剥离出来，这样的一个借助CPU和栈的执行流，我们称之为线程
* 进程和线程的区别：
  * 进程是资源的分配单位，线程是执行的调度单位
  * 进程拥有一个完整的资源平台，线程只独享指令流执行的资源

#### 线程的表示

* 每个线程会包括：
  * 线程`id`：用于唯一确认一个线程
  * 运行栈`stack`：每个线程都有一个独立的运行栈，保存运行时数据
  * 线程执行上下文`context`：线程不在执行时，需要保存其上下文，这样之后才能够将其恢复，继续运行，上下文由`context`类型保存
  * 所属进程的记号`process`：同一个进程中的多个线程，会共享页表、打开文件等信息。因此，我们将它们提取出来放到线程中
  * 内核栈：中断处理拥有的单独的栈
* 赋予线程`Thread`准备执行一个线程`prepare`、发生时钟中断后暂停线程并保存状态`park`、创建一个线程`new`、上锁并获得可变变量inner的引用`inner`的方法
  * `prepare`方法：提供`Thread.process.inner().memory_set.activate()`激活所属的进程的页表，同时将线程执行上下文`context`取出并放至内核栈顶，作用是在发生时钟中断后准备启动一个新的线程
  * `park`方法：检查当前线程的状态`Thread.inner().context`是否为None，同时将提供的状态'context'保存到线程，作用是在发生时钟中断后暂停当前线程，并且保存它的状态`context`
  * `new`方法：调用所属进程的`process.alloc_page_range `方法，让所属进程分配并映射一段连续虚拟空间，同时构建线程的上下文`context`，其中栈顶指针指向新构建的线程栈，然后用`Arc<Thread>`把线程包含的所有信息打包，构建线程
```
/// 线程 ID 使用 'isize'，负数表示错误
pub type ThreadID = isize;
/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程，Process代表进程
    pub process: Arc<Process>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文，当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
}
/// 回顾 Context 结构体定义
pub struct Context {
    pub x:[usize;32],
    pub sstatus:Sstatus,
    pub sepc:usize,
}
```
#### 进程的表示

* 每个进程会包括：
  * 用户态标识`is_user`：区分内核态线程和用户态线程
  * 访存空间`MemorySet`：进程中的线程会共享同一个页表，即可以访问的虚拟内存空间（简称：访存空间），即「它看到的虚拟内存空间分成的内存段」和「这些段中包含的虚拟页到物理页的映射」
* 赋予进程`Process`创建一个内核进程`new_kernel`、上锁并获得可变变量inner的引用`inner`、分配一定数量的连续虚拟空间`alloc_page_range`的方法
  * `new_kernel`创建的访存空间是调用`MemorySet::new_kernel()`方法，创建默认的内核重映射，数据为空
  * `alloc_page_range`方法：从`memory_set`中通过`memory_set.overlap_with()`方法找到给定长度的未占用的虚拟空间，调用`memory_set.add_segment()`方法来分配物理页面并建立映射，最终返回分配的虚拟地址区间
```
/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
}
```

#### 处理器的表示
* 处理器`Processor`：存放和管理线程池
  * 赋予处理器`Processor`获取一个当前线程的 Arc 引用`current_thread`、保存当前线程的上下文`park_current_thread`、激活下一个线程的上下文`prepare_next_thread`、添加一个待执行的线程`add_thread`、唤醒一个休眠线程`wake_thread`、令当前线程进入休眠`sleep_current_thread`、终止当前的线程`kill_current_thread`
    * `park_current_thread`方法：调用`Processor.current_thread()`方法，得到了一个当前线程的 Arc 引用，再调用`thread.park(*context)`方法，把当前线程暂停，并且保存当前线程的`Context`到这个线程中
    
```
lazy_static! {
    /// 全局的 [`Processor`]，这里的 Lock 封装了 spin::Mutex，而在其基础上进一步关闭了中断。这是因为我们（以后）在内核线程中也有可能访问 PROCESSOR，
    /// 但是此时我们不希望它被时钟打断，这样在中断处理中就无法访问 PROCESSOR 了，因为它已经被锁住
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}
```
```
/// 线程调度和管理
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程，休眠线程会从调度器中移除，单独保存，在它们被唤醒之前，不会被调度器安排
    sleeping_threads: HashSet<Arc<Thread>>,
}
```
* process/processor.rs: impl Processor/fn prepare_next_thread：感觉这部分比较重要，贴出代码，仔细分析一下
```
/// 在一个时钟中断时，替换掉 context
pub fn prepare_next_thread(&mut self) -> *mut Context {
    // 向调度器询问下一个线程，调度器 scheduler 具有 Scheduler Trait，定义在 algorithm/src/scheduler/mod.rs，   
    // 其中的 get_next() 方法能够返回下一个等待执行的线程，其余方法不再赘述
    if let Some(next_thread) = self.scheduler.get_next() {
        // 准备启动下一个线程
        let context = next_thread.prepare();
        self.current_thread = Some(next_thread);
        // 返回下一个线程的 context
        context
    } else {
        // 没有活跃线程
        if self.sleeping_threads.is_empty() {
            // 也没有休眠线程，则退出
            panic!("all threads terminated, shutting down");
        } else {
            // 有休眠线程，则等待中断，IDLE_THREAD 代表空闲线程：当所有线程都进入休眠时，切换到这个线程，等待下一次中断
            self.current_thread = Some(IDLE_THREAD.clone());
            IDLE_THREAD.prepare()
        }
    }
}
```
### 线程的创建

* 一个线程开始运行之前的准备工作：
  * 建立页表映射，需要包括以下映射空间：
    * 线程所执行的一段指令
    * 线程执行栈
    * 操作系统的部分内存空间(当发生中断时，需要跳转到 stvec 所指向的中断处理过程。如果操作系统的内存不在页表之中，将无法处理中断)
  * 设置起始执行的地址
  * 初始化各种寄存器，比如 sp
  * 可选：设置一些执行参数（例如 argc 和 argv 等 ）

#### 执行第一个线程

* 为了执行一个线程，我们需要初始化所有寄存器的值，所以首先设计`context`，然后调用`__restore(context)`，`a0`中读取到的就是`context`，所以，`mv sp, a0`就使得`__restore`使用我们设计好的`context`，程序就会直接进入我们的新线程
```
// interrupt/interrupt.asm：
__restore:
    mv      sp, a0  # 加入这一行
```
```
// main.rs: rust_main()
extern "C" {
    fn __restore(context: usize);
}
// 获取第一个线程的 Context
let context = PROCESSOR.lock().prepare_next_thread();
// 启动第一个线程
unsafe { __restore(context as usize) };
unreachable!()
```
* Context ：
  * 通用寄存器：
    * sp：应当指向该线程的栈顶
    * a0-a7：按照函数调用规则，用来传递参数
    * ra：线程执行完跳转的地点
  * sepc
    * 执行 sret 指令后会跳转到这里，所以 sepc 应当存储线程的入口地址（执行的函数地址）
  * sstatus
    * spp 位：保存发生中断前的状态，内核态（1）用户态（0）
    * spie 位：保存中断前是否开中断
    * sie 位：内核态是否允许中断。对用户态而言，无论 sie 取何值都开启中断
  * 中断发生时，系统要切换到内核态。此时，切换前的状态会被保存在 spp 位中。同时，切换前是否开中断，即 sie 位会被保存在 spie 位中，而 sie 位会被置 0，表示关闭中断
  * 中断结束时，执行 sret 指令时，会根据 spp 位的值决定 sret 执行后是处于内核态还是用户态。与此同时，spie 位的值会被写入 sie 位，而 spie 位置 1。这样，特权状态和中断状态就全部恢复了
* 我们在线程开始运行时开启中断，而在操作系统初始化的过程中是不应该有中断的。所以，我们删去之前设置「开启中断」的代码 `sstatus::set_sie();`

### 线程的切换

* 当线程切换时（即时钟中断时），`handle_interrupt`函数需要将上一个线程的`Context`保存起来，然后将下一个线程的`Context`恢复并返回，就可以在中断后跳转到这个线程来执行
  * 不能直接修改`Context`，因为`handle_interrupt`函数返回的`Context`指针除了存储上下文以外，还提供了内核栈的地址

#### 修改中断处理

* 在 Rust 中，引用`&mut`和指针`*mut`只是编译器的理解不同，其本质都是一个存储对象地址的寄存器。这里返回值使用指针而不是引用，是因为其指向的位置十分特殊，其生命周期在这里没有意义
```
/// 中断的处理入口
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
}

/// 处理 ebreak 断点，不切换线程，直接返回原来的上下文（修改一下 sepc）
fn breakpoint(context: &mut Context) -> *mut Context {
    /* ... */
    context
}

/// 执行了两个函数得到的返回值作为下一个线程的上下文
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    /// 保存当前线程的 Context
    PROCESSOR.lock().park_current_thread(context);
    /// 返回下一个线程的 Context
    PROCESSOR.lock().prepare_next_thread()
}
```


### 线程的结束
* 内核线程将自己标记为“已结束”，同时触发一个普通的异常 ebreak。此时操作系统观察到线程的标记，便将其终止
```
/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}
```
```
/// 创建一个内核线程
pub fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // Context 的 set_ra() 方法设置返回地址，线程的返回地址被设置为 kernel_thread_exit ，即它执行的函数完成后便执行 kernel_thread_exit()，可以安全退出
    thread.as_ref().inner().context.as_mut().unwrap().set_ra(kernel_thread_exit as usize);
    thread
}

```

### 内核栈
* 现在，线程保存 Context 都是根据 sp 指针，在栈上压入一个 Context 来存储。但是，对于一个用户线程而言，它在用户态运行时用的是位于用户空间的用户栈。而它在用户态运行中如果触发中断，sp 指针指向的是用户空间的某地址，但此时 RISC-V CPU 会切换到内核态继续执行，就不能再用这个 sp 指针指向的用户空间地址了。这样，我们需要为 sp 指针准备好一个专门用于在内核态执行函数的内核栈。所以，为了不让一个线程的崩溃导致操作系统的崩溃，我们需要提前准备好内核栈，当线程发生中断时可用来存储线程的 Context
* 不是每个线程都需要一个独立的内核栈，因为内核栈只会在中断时使用，而中断结束后就不再使用。在只有一个 CPU 的情况下，不会有两个线程同时出现中断，所以我们只需要实现一个共用的内核栈
* 每个线程都需要能够在中断时第一时间找到内核栈的地址，我们将内核栈的地址存放到内核态使用的特权寄存器 sscratch 中。这个寄存器只能在内核态访问，这样在中断发生时，就可以安全地找到内核栈了

#### 流程
* 预留一段空间作为内核栈
* 运行线程时，在 sscratch 寄存器中保存内核栈指针
* 如果线程遇到中断，则将 Context 压入 sscratch 指向的栈中（Context 的地址为 sscratch - size_of::<Context>()），同时用新的栈地址来替换 sp（此时 sp 也会被复制到 a0 作为 handle_interrupt 的参数）
* 从中断中返回时（__restore 时），a0 应指向被压在内核栈中的 Context。此时出栈 Context 并且将栈顶保存到 sscratch 中

#### 实现

* 为内核栈预留空间：直接使用一个 static mut 来指定一段空间作为内核栈
```
/// 内核栈
#[repr(align(16))]
#[repr(C)]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);

/// 公用的内核栈
pub static mut KERNEL_STACK: KernelStack = KernelStack([0; STACK_SIZE]);

impl KernelStack {
    /// 在栈顶加入 Context 并且返回新的栈顶指针
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // 栈顶
        let stack_top = &self.0 as *const _ as usize + size_of::<Self>();
        // Context 的位置
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}
```
* 线程遇到中断时，切换到内核栈，再将 Context 压入栈中
```
// interrput/interrupt.asm
__interrupt:
    # 启动第一个线程时，context 是内核栈的新的栈顶指针，作为 _restore 的参数 a0 ，sscratch 在 _restore 里被写入
    # 交换 sp 和 sscratch（切换到内核栈）
    csrrw   sp, sscratch, sp
    # 在内核栈开辟 Context 的空间
    addi    sp, sp, -36*8

    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    # 将本来的栈地址 sp（即 x2）保存
    csrr    x1, sscratch
    SAVE    x1, 2
```
* 从中断中返回时，a0 应指向被压在内核栈中的 Context，此时出栈 Context ，恢复所有寄存器，并且将栈顶保存到 sscratch 中 
``` 
__restore:
    # 从 a0 中读取 sp，启动第一个线程时，a0 是参数 context 直接赋值的，之后的线程，handle_interrupt 函数的返回值赋值给 a0，如果是时钟中断，则指向的是下一个线程的 Context
    mv      sp, a0
    # 恢复 CSR
    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1
    # 将内核栈地址写入 sscratch
    addi    t0, sp, 36*8
    csrw    sscratch, t0
```
### 运行

```
pub extern "C" fn rust_main() -> ! {
    memory::init();
    interrupt::init();
    {
        let mut processor = PROCESSOR.lock();
        // 创建一个内核进程
        let kernel_process = Process::new_kernel().unwrap();
        // 为这个进程创建多个线程，并设置入口均为 sample_process，而参数不同
        for i in 1..9usize {
            processor.add_thread(create_kernel_thread(
                kernel_process.clone(),  // 指所属的进程
                sample_process as usize,  // 指入口地址，线程在执行完 sample_process 后会返回到 ra 指向的地址，即 kernel_thread_exit，这里把当前线程标记为结束，操作系统将其终止
                Some(&[i]),   
            ));
        }
    }
    extern "C" {
        fn __restore(context: usize);
    }
    // 获取第一个线程的 Context，返回值 context 是把第一个线程的 Context 放入内核栈之后的栈顶指针
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程，这里通过参数 context 给 sscratch 赋值，即 sscatch 已经保存内核栈指针，而中断一直在进行，每次时钟中断到达，保存上一个线程的 Context 之后，       
    // 就会调用 handle_interrupt 函数，如果当前线程已经执行完 sample_process，即当前线程已经标记为结束，会产生相应的输出，并且准备下一个线程，如果当前线程还没结束，   
    // 就判断中断原因，进行对应的操作，直到下一个线程已经不存在，也就是 prepare_next_thread 函数中，判断没有活跃进程和休眠进程的情况，则退出
    unsafe { __restore(context as usize) };
    unreachable!()
}

fn sample_process(message: usize) {
    println!("hello from kernel thread {}", id);
}
```
### 总结
* 这几天事情有点多，这个Lab4的战线就拉的有点长，花了好几天才完全学完，感觉理解起来是比较困难的一个实验了，按照文档的顺序，先介绍了进程、线程、处理器的概念，我一般会对照rCore-Tutorial的代码，直接先去理解每个结构体具有的方法，当时就理解有点困难，后来尝试着翻了一下后面的文档，才发现这些结构体的方法在后面会有详细的介绍，我也就完全按照文档的顺序往下走了，在文档讲到结构体的方法时，再根据rCore-Tutorial的代码进行相应补充，学习效率也得以提高。当时Lab1理解中断的进行流程就有点困难，这里在理解main函数的流程时，刚开始也遇到了困难，就只能是反复看那些代码，从一个文件的函数跳到另一个文件的函数那样去对照，基本总结出来了运行的流程

## Lab5学习报告

### 设备树

* 在 RISC-V 中，一般是由 bootloader，即 OpenSBI 固件完成对于包括物理内存在内的各外设的扫描，将扫描结果以设备树二进制对象设备树`DTB(Device Tree Blob)`的格式保存在物理内存中的某个地方，而这个放置的物理地址将放在 a1 寄存器中，而硬件线程，可以理解为执行的 CPU 核`HART ID(HART，Hardware Thread)`放在 a0 寄存器上
```
// 如果要使用这两个参数，不需要修改任何入口汇编的代码，只需要给 rust_main 函数增加两个参数即可
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {}
```

* 设备树结构：每个设备在物理上连接到了父设备上最后再通过总线等连接起来构成一整个设备树，在每个节点上都描述了对应设备的信息，而操作系统就是通过这些节点上的信息来实现对设备的识别的
* 对于设备节点属性，我们不需要自己来实现这件事情，可以直接调用 rCore 中 device_tree 库，然后遍历树上节点即可，设备节点属性包括：
  * compatible：指的是该设备的编程模型，一般格式为 "manufacturer,model"，分别指一个出厂标签和具体模型。如 "virtio,mmio" 指的是这个设备通过 virtio 协议、MMIO（内存映射 I/O）方式来驱动
  * model：指的是设备生产商给设备的型号
  * reg：当一些很长的信息或者数据无法用其他标准属性来定义时，可以用 reg 段来自定义存储一些信息
  
* 解析设备树：
  * `walk`函数：递归遍历设备树
    * 检查设备的协议支持，一旦发现了一个支持`"virtio,mmio"`的设备（其实就是 QEMU 模拟的存储设备），就进入下一步加载驱动的逻辑，然后遍历子树
  * `init`函数：遍历设备树并初始化设备
    * 首先验证`Magic Number`，与设备树格式的`DEVICE_TREE_MAGIC`比较，这一步是一个保证系统可靠性的要求，是为了验证这段内存到底是不是设备树，然后拷贝数据并加载，再调用`walk`函数遍历设备树
  
### virtio
  
* virtio 是一种 I/O 半虚拟化解决方案，是一套通用 I/O 设备虚拟化的程序，提供了一套上层应用与各 Hypervisor 虚拟化设备（KVM，Xen，VMware等）之间的通信框架和编程接口

* 在完全虚拟化的解决方案中，guest VM 要使用底层 host 资源，需要 Hypervisor 来截获所有的请求指令，然后模拟出这些指令的行为，这样势必会带来很多性能上的开销。半虚拟化通过底层硬件辅助的方式，将部分没必要虚拟化的指令通过硬件来完成，Hypervisor 只负责完成部分指令的虚拟化，要做到这点，需要 guest 来配合，guest 完成不同设备的前端驱动程序，Hypervisor 配合 guest 完成相应的后端驱动程序，这样两者之间通过某种交互机制就可以实现高效的虚拟化过程，virtio 就是一套通用框架和标准接口（协议）来完成两者之间的交互过程

#### 架构

从总体上看，virtio 可以分为四层，包括前端 guest 中各种驱动程序模块，后端 Hypervisor （实现在Qemu上）上的处理程序模块，中间用于前后端通信的 virtio 层和 virtio-ring 层，virtio 这一层实现的是虚拟队列接口，算是前后端通信的桥梁，而 virtio-ring 则是该桥梁的具体实现，它实现了两个环形缓冲区，分别用于保存前端驱动程序和后端处理程序执行的信息

* 前端
  
|virtio_blk           |virtio_net           |virtio_pci           |virtio_balloon       |virtio_scsi          |virtio_console       |
|---------------------|---------------------|---------------------|---------------------|---------------------|---------------------|
  
* 中间层
  
|virtio                                                                                                                             |  
|------------------------------------------------------------------------------------------------------------------------------------|

|transport(virtio_ring)                                                                                                                          |
|------------------------------------------------------------------------------------------------------------------------------------|

* 后端
 
|virtio backend                                                                                                                       |
|------------------------------------------------------------------------------------------------------------------------------------|

#### virtio 节点探测
* `virtio_probe`函数：从设备树的某个节点探测 virtio 协议具体类型
  * 设备树节点的 reg 信息中可以读出设备更详细信息的放置位置（如：在 0x10000000 - 0x10010000 ），这段区间虽然算是内存区间，但是我们的物理内存只分布在 0x80000000 到 0x88000000 的空间中，这段空间是所谓的内存映射读写 MMIO（Memory Mapped I/O），也就是总线把对设备操作信息传递也映射成了内存的一部分，即通过将外围设备映射到内存空间，便于 CPU 的访问
* rCore 中的 virtio_drivers 库帮我们通过 MMIO 的方式对设备进行交互，同时我们也需要给这个库提供一些诸如申请物理内存、物理地址虚拟转换等接口
  * `virtio_dma_alloc`函数：为 DMA 操作申请连续 pages 个物理页（为 [`virtio_drivers`] 库提供）
  * `virtio_dma_dealloc`函数：为 DMA 操作释放对应的之前申请的连续的物理页（为 [`virtio_drivers`] 库提供）
  * `virtio_phys_to_virt`函数：将物理地址转为虚拟地址（为 [`virtio_drivers`] 库提供）
    * 物理地址到虚拟地址转换直接线性映射，因为在内核重映射的时候，我们已经把全部的段放进去了，所以物理地址直接加上 Offset 得到的虚拟地址是可以通过任何内核进程的页表来访问的
  * `virtio_virt_to_phys`函数：将虚拟地址转为物理地址（为 [`virtio_drivers`] 库提供）
    * 虚拟地址到物理地址需要查页表，因为内核栈是以 Frame 为单位分配的，而以 Frame 为单位分配意味着，虚拟地址可能从 0 开始，这个时候要转为物理地址，显然不是减去偏移量的线性映射，而必须查当前的表
    
### 驱动和块设备驱动

#### 抽象驱动

```
/// 驱动的接口
pub trait Driver: Send + Sync {
    /// 设备类型，目前只有块设备 Block ，可能还有网络、GPU 设备等
    fn device_type(&self) -> DeviceType;

    /// 读取某个块到 buf 中（块设备接口）
    fn read_block(&self, _block_id: usize, _buf: &mut [u8]) -> bool {}

    /// 将 buf 中的数据写入块中（块设备接口）
    fn write_block(&self, _block_id: usize, _buf: &[u8]) -> bool {}
}
```
```
lazy_static! {
    /// 所有驱动
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
}
```
#### 抽象块设备

* 为块设备 BlockDevice 实现 rcore-fs 中提供的 BlockDevice trait，使得文件系统可以通过调用块设备的该接口来读写，实现了为文件系统的接口，实际上是对上传文件系统的连接
  * 每个块的大小取 2 的对数，这里取 512B 是因为 virtio 驱动对设备的操作粒度为 512B ，即`const BLOCK_SIZE_LOG2: u8 = 9;`
  * 赋予块设备`read_at`方法：读取某个块到 buf 中
    * 这里调用了`BlockDevice.0.read_block`
  * 赋予块设备`write_at`方法：将 buf 中的数据写入块中
    * 这里调用了`BlockDevice.0.write_block`
  * 赋予块设备`sync`方法：执行和设备的同步
```
/// 块设备抽象（驱动的引用）
pub struct BlockDevice(pub Arc<dyn Driver>);
```
#### virtio-blk 块设备驱动
* 为 VirtIOBlkDriver 实现 Driver trait，调用了 virtio_drivers 库
* `add_driver`函数：将从设备树中读取出的设备信息放到 static@DRIVERS 中
```
/// virtio 协议的块设备驱动
struct VirtIOBlkDriver(Mutex<VirtIOBlk<'static>>);
```
#### 设计模式

* `struct VirtIOBlkDriver(Mutex<VirtIOBlk<'static>>)` 对上实现了`trait Driver(驱动功能的抽象)`，`BlockDevice(pub Arc<dyn Driver>)(设备的抽象)`是对驱动的引用，设备再实现对上层功能的要求`trait BlockDevice(文件系统)`
* `Driver`作为一个核心`trait`为上提供实现，上层也就是`Driver`的使用侧`BlockDevice`（设备的抽象），而下层则是`Driver`的实现侧`VirtIOBlkDriver`（设备的实现）

### 文件系统

* 在 QEMU 运行的时候加入选项，选择 QEMU 支持的 virtio 协议，我们引入了一个磁盘镜像文件`TEST_IMG`，这个文件的打包是由`rcore-fs-fuse`工具来完成的。`rcore-fs-fuse`工具会根据不同的格式把目录的文件封装到一个文件系统中，并把文件系统封装为一个磁盘镜像文件，然后我们把这个镜像文件像设备一样挂载在 QEMU 上，QEMU 就把它模拟为一个块设备了
```
# 运行 QEMU
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000 \
            -drive file=$(TEST_IMG),format=raw,id=sfs \      # 模拟存储设备
            -device virtio-blk-device,drive=sfs              # 以 virtio Block Device 的形式挂载到 virtio 总线上
```        
* 要让操作系统理解块设备里面的文件系统，要存取根目录的`INode`(对一个文件的位置抽象，目录也是文件的一种)，后面对于文件系统的操作都可以通过根目录来实现，而我们要找到全部设备驱动中的第一个存储设备作为根目录
```
lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                // 动态分配一段内存空间作为设备 Cache
                // BlockCache::new(device, BLOCK_CACHE_CAPACITY) 可以把 device 自动变为一个有 Cache 的设备，块设备的 Cache 块个数是 BLOCK_CACHE_CAPACITY
                let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                return SimpleFileSystem::open(device_with_cache)
                    .expect("failed to open SFS")
                    .root_inode();
            }
        }
        panic!("failed to load fs")
    };
}
```
* 触发 ROOT_INODE 的初始化，即可访问根文件目录下的内容，进行创建、遍历等操作
```
/// 测试
pub fn ls(path: &str) {
    let mut id = 0;
    let dir = ROOT_INODE.lookup(path).unwrap();
    print!("files in {}: \n  ", path);
    /* ... */
}
```
### 总结

* Lab5的实现主要是在调用各种接口，或者现成的库，所以代码量不是很大，但是因为对这些接口不了解，所以代码阅读起来不是太顺利，而且开始会觉得文档的各个章节之间有点散，联系不太起来，后来看到最后文件系统的实现，有点豁然开朗，整个文件系统的实现逻辑就是：在 QEMU 上挂载存储设备 -> 找到全部设备驱动中的第一个存储设备作为根目录 -> 通过根目录来实现对于文件系统的操作
