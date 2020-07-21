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
