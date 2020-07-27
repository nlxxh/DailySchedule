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
* 赋予进程`Process`创建一个内核进程`new_kernel`、根据文件创建进程`from_elf`、上锁并获得可变变量inner的引用`inner`、分配一定数量的连续虚拟空间`alloc_page_range`的方法
  * `new_kernel`创建的访存空间是调用`MemorySet::new_kernel()`方法，创建默认的内核重映射，数据为空
  * `from_elf`创建的访存空间是调用`MemorySet::from_elf()`方法，通过提供的elf文件，创建内核重映射，每个字段的大小、地址、数据均由elf文件确定
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
    /// 全局的 [`Processor`]，这里的 Lock 封装了 spin::Mutex，而在其基础上进一步关闭了中断。这是因为我们（以后）在内核线程中也有可能访问 PROCESSOR，但是此时我们不希望它被时钟打断，这样在中断处理中就无法访问 PROCESSOR 了，因为它已经被锁住
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
    // 向调度器询问下一个线程，调度器 scheduler 具有 Scheduler Trait，定义在 algorithm/src/scheduler/mod.rs，其中的 get_next() 方法能够返回下一个等待执行的线程
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

/// 处理 ebreak 断点，直接返回原来的上下文（修改一下 sepc）
fn breakpoint(context: &mut Context) -> *mut Context {
    /* ... */
    context
}

/// 处理时钟中断，执行了两个函数得到的返回值作为下一个线程的上下文
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    /// 保存当前线程的 Context
    PROCESSOR.lock().park_current_thread(context);
    /// 返回下一个线程的 Context
    PROCESSOR.lock().prepare_next_thread()
}
```
* 现在，线程保存 Context 都是根据 sp 指针，在栈上压入一个 Context 来存储。但是，对于一个用户线程而言，它在用户态运行时用的是位于用户空间的用户栈。而它在用户态运行中如果触发中断，sp 指针指向的是用户空间的某地址，但此时 RISC-V CPU 会切换到内核态继续执行，就不能再用这个 sp 指针指向的用户空间地址了。这样，我们需要为 sp 指针准备好一个专门用于在内核态执行函数的内核栈。所以，为了不让一个线程的崩溃导致操作系统的崩溃，我们需要提前准备好内核栈，当线程发生中断时可用来存储线程的 Context

