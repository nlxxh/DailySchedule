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
