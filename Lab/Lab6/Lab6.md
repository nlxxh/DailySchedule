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

* 
