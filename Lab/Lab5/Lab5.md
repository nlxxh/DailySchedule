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
