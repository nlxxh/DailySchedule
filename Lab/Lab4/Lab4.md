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
  * `prepare`方法：提供`Thread.process.inner().memory_set.activate()`激活所属的进程的页表，同时将线程执行上下文`context`放至内核栈顶
  * `park`方法：检查当前线程的状态`Thread.inner().context`是否为None，同时将提供的状态'context'保存到线程
  * `new`方法：调用所属进程的`process.alloc_page_range `方法，让所属进程分配并映射一段连续虚拟空间，作为线程的栈，同时构建线程的上下文`context`，其中栈顶指针指向新构建的线程栈，然后用`Arc<Thread>`把线程的信息打包，构建线程
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
* 
```
/// 线程调度和管理
///
/// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}
```
