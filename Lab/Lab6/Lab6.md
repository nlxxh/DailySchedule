## Lab6学习报告

### 构建用户程序框架

* 我们要为用户程序提供一个类似的没有 Rust std 标准运行时依赖的极简运行时环境

```
// 目录结构
rCore-Tutorial
  - os
  - user
    - src
      - bin  // bin 中存放的源文件会被编译成多个单独的执行文件
        - hello_world.rs
      - lib.rs  // lib 中存放极简运行时环境
      - console.rs  // 实现 print! println! 宏以及从控制台读取字符
    - Cargo.toml
    - .cargo
      - config  // 设置编译目标为 RISC-V 64
    - Makefile
```
* lib.rs
  * #![no_std] 移除标准库
  * #![feature(...)] 开启一些不稳定的功能
  * #[global_allocator] 使用库来实现动态内存分配
  * #[panic_handler] panic 时终止

### 打包为磁盘镜像

* 利用`rcore-fs-fuse`工具将编译后的用户程序打包为镜像，即把包含编译生成的`ELF`文件的目录打包成`SimpleFileSystem`格式的磁盘镜像，然后将镜像文件的格式转换为`QEMU`使用的高级格式，就可以让操作系统加载磁盘镜像
```
// user/Makefile：
build: dependency
    # 编译
    @cargo build
    @echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
    # 移除原有的所有文件
    @rm -rf $(OUT_DIR)
    @mkdir -p $(OUT_DIR)
    # 复制编译生成的 ELF 至目标目录
    @cp $(BIN_FILES) $(OUT_DIR)
    # 使用 rcore-fs-fuse 工具进行打包
    @rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
    # 将镜像文件的格式转换为 QEMU 使用的高级格式
    @qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
    # 提升镜像文件的容量（并非实际大小），来允许更多数据写入
    @qemu-img resize $(QCOW_FILE) +1G
```

### 解析 ELF 文件并创建线程

* 利用`xmas-elf`解析器解析`ELF`文件，从`ELF`文件中加载用户程序的代码和数据信息，并且映射到内存中

#### 读取文件内容

```
// os/src/fs/inode_ext.rs:
/// 为 INode 类型添加的扩展功能
pub trait INodeExt {
    /// 打印当前目录的文件
    fn ls(&self);
    /// 读取文件内容
    fn readall(&self) -> Result<Vec<u8>>;
}
impl INodeExt for dyn INode {
    /*...*/
    // 将整个文件作为 [u8] 读取出来
    fn readall(&self) -> Result<Vec<u8>> {
        // 从文件头读取长度
        let size = self.metadata()?.size;
        // 构建 Vec 并读取
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size) };
        self.read_at(0, buffer.as_mut_slice())?;
        Ok(buffer)
    }
}
```
#### 解析各个字段
* 利用`xmas-elf`库中的接口，从读出的`ELF`文件中对应建立`MemorySet`
  * `from_elf`方法：首先调用`MemorySet::new_kernel()`创建内核重映射，然后遍历`elf`文件的所有部分，在`elf`文件的每个部分中读取到字段的「起始地址」「大小」和「数据」，将字段的这些信息组织起来作为`Segment`进行映射，然后调用`MemorySet.add_segment()`，把这个`Segment`加入`MemorySet`，建立映射并复制数据
```
// os/src/memory/mapping/memory_set.rs：
/// 通过 elf 文件创建内存映射（不包括栈）
pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<MemorySet> {
/*...*/
}
```
#### 加载数据到内存中

* 为用户程序建立映射时，虚拟地址是 ELF 文件中写明的，但是物理地址不是程序在磁盘中存储的地址，因为如果直接映射磁盘空间，使用时会带来巨大的延迟，所以需要在程序准备运行时，将其磁盘中的数据复制到内存中。如果程序较大，操作系统可能只会复制少量数据，而更多的则在需要时再加载。当然，我们实现的简单操作系统就一次性全都加载到内存中了

* `MemorySet.add_segment()`方法调用了 `MemorySet.mapping.map()`方法，即建立虚实地址的映射，这里需要分配页面再进行映射。对于一个页面，有其物理地址、虚拟地址和待加载数据的地址，不是从待加载数据的地址拷贝到页面的虚拟地址，因为，在目前的框架中，只有当线程将要运行时，才会加载其页表。因此，除非我们额外的在每映射一个页面之后，就更新一次页表并且刷新 TLB，否则此时的虚拟地址是无法访问的。但是，我们通过分配器得到了页面的物理地址，而这个物理地址实际上已经在内核的线性映射当中了。所以，这里实际上用的是物理地址来写入数据

### 实现系统调用
