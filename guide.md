# OS实验的测试要求(v0.1)

### 概述

实现指导见 rcore_tutorial_book_v3，该文档仅解释测试程序。在实现文件系统之前，你应当以合适的方式（可参考知道书或者样例实现）将测试文件置于内存中某一个位置并设法运行。

### 测试文件说明

在 `/user` 目录下 `make chx` 即可得到实验x的测例，位于 `user/build`目录下，`elf` 和 `bin` 表示格式。（具体操作见 `user/Makefile`）

`chx_*` 格式的文件表明属于实验x，`chxt_*` 格式的文件表明属于实验x，但仅仅是暂时实现，后续可能会去除，具体见下方描述。

## lab1

##### 核心目标

boot 起来，能够输出就行。

##### 实现

完成基本初始化，主要是硬件加电后的硬件初始化，推荐使用SBI。OS需要知道内存大小，IO分布。

实现串口输出功能，方便后续调试（可依赖SBI）。

##### 测试

模仿样例实现

* 获取内存布局并输出

  * 例如

    ```rust
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn boot_stack();
            fn boot_stack_top();
        };
        clear_bss();
        println!("Hello, world!");
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!("boot_stack [{:#x}, {:#x})", boot_stack as usize, boot_stack_top as usize);
        println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    ```

* 实现彩色化输出

  * 例如：

    ```Rust
    debug!("{}{}{}", a, b, c);
    info!("{}{}{}", c, d, e);
    error!("{}{}{}", A, B, C);
    ```

    可以参考[rCore](https://github.com/rcore-os/rCore.git) ，[垃圾os](https://github.com/DeathWish5/appdir)。
    
  * 本质是在print的内容之前/之后输出一段特殊的控制字符。

## lab2

##### 核心目标

实现内核/用户态隔离与切换。

##### 硬性约定

* 用户程序用户栈大小为一个页，也就是0x1000 (4k)，且按照0x1000对其。统一大小方便测试。

##### 实现

* 隔离
  * 功能描述：U态不能访问 M/S 态的指令和寄存器，程序不能访问非法地址
  * 对应测例：`ch2[t]_bad_*`。暂时性说明：`ch2t_bad_address` 在lab4实现虚存后失效。

* `sys_write`:
  * 功能描述：与 posix `sys_write` 基本一致
    * syscall ID：64
    * 功能：从内存缓冲区写入一段内容到文件/串口。
    * C 接口：`int sys_write(int fd, char *buf, int len)`;
    * Rust 接口：`fn sys_write(fd: i32, buf: *mut u8, len: i32) -> i32`;
    * 参数：**fd** 描述当前进程需要访问的文件，**buf** 表示保存即将写入文件的数据的缓冲区的地址，**len** 表示最大的写入字节数。
    * 返回值：如果出现了错误则返回 -1，否则返回实际写入的字节数。
    * 可能的错误：
      * 传入的 **fd** 不合法（目前仅支持 stdout）
      * 传入缓冲区位于用户地址之外（需要分别检查 .text .data .bss 各段以及用户栈，如果是 bin 格式会简单很多）
    * 备注：该 syscall 的实现可能是阻塞的。
  * 对应测例：
    * `ch2t_write0`: 测试错误检查。暂时性说明：在lab4实现虚存后失效。
    * `ch2_write1`: 测试参数正确时的功能实现。
* `sys_exit`:
  * 功能描述：与 posix `sys_exit` 基本一致
    * syscall ID：93
    * 功能：退出当前进程。
    * C 接口：`int sys_exit(int status);`
    * Rust 接口：`fn sys_exit(status: i32) -> i32;`
    * 参数：**status** 描述当前进程的返回值，并应当由其父进程捕获到。
    * 返回值：正常情况下应不会返回。请在调用 exit 之后加入 panic 语句来确保这一点。
    * 可能的错误：没啥。
  * 对应测例：
    * `ch2_exit`
    * 所有用户程序最终都需要调用

## lab3

##### 核心目标

实现最简单的上下文的概念，可以切换进程。实现时钟中断。

##### 硬性约定

* 进程优先级在 [2, i64_max] 之间。注意，这里的优先级不能为1和0是为了实现 stride 调度的方便。
* 仅在当前实验中，给用户程序设定一个较大的运行时间上限，超出就杀死，这是为了确保实现了时钟中断。暂时性说明：这个单纯是为了通过死循环测试，lab4开始就删掉。

##### 使用说明

需要分别 make ch3_0/ch3_1/ch3_2 并执行测试。这是为了是的调度特征更加明显。

##### 实现

* `sys_gettime`
  * 功能描述：与 posix `sys_gettime` 一致
    * syscall ID：169
    * 功能：获取当前时间。
    * C 接口：`int gettime(TimeVal* ts, int tz)`;
    * Rust 接口：`fn gettime(ts: &TimeVal, tz: usize) -> isize`;
    * 参数：
      * ts 为当前时间结构体
      ```rust
      #[repr(C)]
      #[derive(Debug)]
      pub struct TimeVal {
          pub sec: usize,
          pub usec: usize,
      }
      ```
      * tz 表示时区，这里无需考虑，始终为0。
    * 返回值：正确返回 0，错误返回 -1。
    * 可能的错误：无。
  * 对应测例：
    
    * `ch3_0_sleep`：注意这里没有实现sleep系统调用，只是一个用户太模拟。
  
* `sys_yield`
  * 功能描述：与 posix `sys_yield` 基本一致
    * syscall ID：124
    * 功能：主动交出当前进程的 CPU 使用权，从而使得 CPU 可以执行其他进程。
    * C 接口：`int yield();`
    * Rust 接口：`fn yield() -> i32;`
    * 参数：无参数。
    * 返回值：总是返回 0。
    * 可能的错误：没有正确切换。
  * 对应测例
    * `ch3_1_yield*`: 测试时需要以批处理形式并发运行`ch3_1_yield[012]`，正确输出为交替出现的 ABC。为了排除其他程序干扰，推荐不要运行除此之外的程序。
* `sys_set_priority`
  * 功能描述：设定进程优先级
    * syscall ID: 140
    * 功能：设定进程优先级。
    * C 接口：`int setpriority(long long prio);`
    * Rust 接口：`fn setpriority(prio: isize) -> isize;`
    * 说明：设定自身进程优先级，只要 prio 在 [2, isize_max] 就成功，返回 prio，否则返回 -1。
  * 对应测例
    * `ch3_0_setprio`
  
* 时钟中断
  * 功能描述：定期的时钟中断
  * 对应测试：
    * `ch3_t_deadloop`: 就是一个死循环，只有实现了时钟中断才能把它杀死。
  
* stride 调度
  * 功能描述：见指导书对应章节。
  * 测试说明：
    * `ch3_3_stride`: 测试时需要以批处理形式并发运行`ch3_3_stride[012345]`，程序本质是过固定时间计数+1，最终输出的计数要和优先级基本成正比。

## lab4

##### 核心目标

实现虚存/物理内存管理。

##### 硬性约定

* 注意删除　lab3　中杀死进程的逻辑
* 该章节 `mmap`系统调用为非标准格式，含义也不尽相同。

##### 实现

* mmap 
  * syscall ID：222
  * C接口：`int mmap(void* start, unsigned long long len, int port);`
  * Rust接口：`fn mmap(start: usize, len: usize, port: usize) -> isize;`
  * 功能：申请长度为 len 字节的物理内存，并映射到 addr 开始的虚存。
  * 参数：
    * start 需要映射的虚存起始地址，要求按页对齐。
    * len：映射字节长度，可以为0（如果是则直接返回），暂时不考虑 len 过长的情况。实现中对页长度取上整。
    * port：第0位表示是否可读，第1位表示是否可写，第2位表示是否可执行。其他位无效。
  * 说明：正确返回实际 map size（为 4096 的倍数），错误返回 -1。为了简单：
    * start 若非按页对其按错误处理
    * 暂时不考虑分配失败时的物理内存回收（也就是内存泄漏）
  * 错误： 
    * [start, start + len) 中部分或者全部已经映射。
    * 物理内存不足。

* munmap
  * syscall ID：215
  * C接口：`int mmap(void* start, unsigned long long len);`
  * Rust接口：`fn mmap(start: usize, len: usize) -> i32;`
  * 功能：取消一块虚存的映射。
  * 参数：同 mmap。
  * 说明：正确返回实际 unmap size（为 4096 的倍数），错误返回 -1。为了简单：
    * addr 若非按页对其按错误处理
    * 暂时不考虑中途失败时的物理内存回收与恢复
  * 错误： 
    * [start, start + len) 中部分或者全部没有被映射。

## lab5

##### 核心目标

实现进程管理。

##### 硬性约定

* 该章节 `sys_spawn`系统调用为生造，实际上没有该系统调用，但是可以用 fork + exec 模拟。

##### 实现

* getpid
  * syscall ID：172
  * 功能：获取当前进程的进程 ID。
  * C 接口：`int getpid();`
  * Rust 接口：`fn getpid() -> i32;`
  * 参数：无参数。
  * 返回值：返回当前进程的进程 ID。
  * 可能的错误：无。

* spawn
  * syscall ID: 400
  * C 接口：`int spawn(char *file);`
  * Rust 接口：`fn spawn(file: *const u8);`
  * 功能：创建一个子进程并执行目标路径文件，暂时不考虑参数，不要求立即开始执行，相当于 fork + exec。
  * 说明：成功返回子进程 id，否则返回 -1。

* waitpid
  * syscall ID：260
  * 功能：当前进程等待一个子进程结束，并获取其返回值。
  * C 接口：`int waitpid(int pid, int *status);`
  * Rust 接口： `fn waitpid(pid: i32, status: *mut i32) -> i32;`
  * 参数：
    * **pid** 表示要等待结束的子进程的进程 ID，如果为 0或者-1 的话表示等待任意一个子进程结束；
    * **status** 表示保存子进程返回值的地址，如果这个地址为 0 的话表示不必保存。
  * 返回值：如果出现了错误则返回 -1；否则返回结束的子进程的进程 ID。
  * 说明: 
    * 该 syscall 会导致阻塞。
  * 可能的错误：
    * 进程无未结束子进程。
    * pid 非法或者指定的不是该进程的子进程。
    * 传入的地址 status 不为 0 但是不合法；

## lab6

##### 核心目标

实现基于邮箱的进程间通信。

#####　硬性约定

* 该章节系统调用为生造，实际上没有真实的系统调用。

* 邮箱说明：

  每个进程默认拥有唯一一个邮箱，基于“数据报文”收发字节信息，利用环形buffer存储，读写顺序为 FIFO，不记录来源进程。每次读写单位必须为一个报文，如果缓冲区长度不够，舍弃超出的部分（也就是截断报文）。为了简单，邮箱中最多拥有16条报文，每条报文最大长度256字节。当邮箱满时，发送邮件（也就是写邮箱会失败）。不考虑读写邮箱的权限，也就是所有进程都能够随意读写其他进程的邮箱。

##### 实现

* mailread
  * syscall ID：401
  * C接口：`int mailread(void* buf, int len)`
  * Rust接口: `fn mailread(buf: *mut u8, len: usize);`
  * 功能：读取一个报文，如果成功返回报文长度.
  * 参数：buf: 缓冲区头。len：缓冲区长度。
  * 说明：
    * len > 256 按 256 处理，len < 队列首报文长度且不为0，则截断报文。
    * len = 0，则不进行读取，如果没有报文读取，返回-1，否则返回0，这是用来测试是否有报文可读。
  * 可能的错误：
    * 邮箱空。
    * buf 无效。
* mailwrite
  * syscall ID：402
  * C接口：`int mailwrite(int pid, void* buf, int len)`
  * Rust接口: `fn mailwrite(pid: usize, buf: *mut u8, len: usize);`
  * 功能：向对应进程邮箱插入一条报文.
  * 参数：pid: 目标进程id。buf: 缓冲区头。len：缓冲区长度。
  * 说明：
    * len > 256 按 256 处理，
    * len = 0，则不进行写入，如果邮箱满，返回-1，否则返回0，这是用来测试是否可以发报。
    * 可以向自己的邮箱写入报文。
  * 可能的错误：
    * 邮箱满。
    * buf 无效。

## lab7

##### 核心目标

实现磁盘简单读写与管理。

#####　硬性约定

* 本章为扩展实验，请先理解参考代码并实现（可以拷贝代码）基础文件管理和读写。
* 暂不考虑删除文件，不可以使用 unlink 删除文件。

##### 实现

* open

  * syscall ID：56

  * 功能：打开一个文件，并返回可以访问它的文件描述符。

  * C 接口：`int open(int dirfd, char* path, unsigned int flags, unsigned int mode);`

  * Rust 接口：`fn open(dirfd: usize, path: *const u8, flags: u32, mode: u32);`

  * 参数：

    * **dirfd**: 仅为了兼容性考虑，本次实验中始终为 AT_FDCWD (-100)。可以忽略。

    * **path** 描述要打开的文件的文件名（简单起见，文件系统不需要支持目录，所有的文件都放在根目录 / 下），

    * **flags** 描述打开文件的标志，具体含义（其他参数不考虑）：

      ```c
      #define O_RDONLY  0x000
      #define O_WRONLY  0x001
      #define O_RDWR    0x002		// 可读可写
      #define O_CREATE  0x200
      ```

    * **mode** 仅在创建文件时有用，表示传建文件的访问权限，为了简单，本次实验中可以忽略。

  * 说明：

    * 有 create 标志但文件存在时，忽略 create 标志，直接打开文件。

  * 返回值：如果出现了错误则返回 -1，否则返回可以访问给定文件的文件描述符。

  * 可能的错误：

    * 文件不存在且无 create 标志。
    * 标志非法（低两位为 0x3）
    * 打开文件数量达到上限。

* close

  * syscall ID：57
  * 功能：关闭一个文件。
  * C 接口：`int close(int fd);`
  * Rust 接口：`fn close(fd: i32) -> i32;`
  * 参数：**fd** 为文件描述符。
  * 返回值：如果出现了错误则返回 -1，否则返回 0。
  * 可能的错误：
    * 传入的文件描述符 fd 并未被打开或者为保留句柄。

* link

  * syscall ID: 37
  * 功能：创建一个文件的一个硬链接，含义课堂讲授。
  * Ｃ接口：`int linkat(int olddirfd, char* oldpath, int newdirfd, char* newpath, unsigned int flags)`
  * Rust 接口：`fn linkat(olddirfd: i32, oldpath: *const u8, newdirfd: i32, newpath: *const u8, flags: u32) -> i32`
  * 参数：
    * olddirfd，newdirfd: 仅为了兼容性考虑，本次实验中始终为 AT_FDCWD (-100)，可以忽略。
    * flags: 仅为了兼容性考虑，本次实验中始终为 0，可以忽略。
    * oldpath：原有文件路径
    * newpath: 新的链接文件路径。
  * 说明：
    * 为了方便，不考虑新文件路径已经存在的情况（属于未定义行为），除非链接同名文件。
  * 返回值：如果出现了错误则返回 -1，否则返回 0。
  * 可能的错误
    * 链接同名文件。

* unlink

  * syscall ID: 35
  * 功能：取消一个文件路径到文件的链接。
  * Ｃ接口：`int unlinkat(int dirfd, char* path, unsigned int flags)`
  * Rust 接口：`fn unlinkat(dirfd: i32, path: *const u8, flags: u32) -> i32`
  * 参数：
    * dirfd: 仅为了兼容性考虑，本次实验中始终为 AT_FDCWD (-100)，可以忽略。
    * flags: 仅为了兼容性考虑，本次实验中始终为 0，可以忽略。
    * path：文件路径。
  * 说明：
    * 为了方便，不考虑使用 unlink 彻底删除文件的情况。
  * 返回值：如果出现了错误则返回 -1，否则返回 0。
  * 可能的错误
    * 文件不存在。

* fstat

  * syscall ID: 80

  * 功能：获取文件状态。

  * Ｃ接口：`int fstat(int fd, struct Stat* st)`

  * Rust 接口：`fn fstat(fd: i32, st: *mut Stat) -> i32`

  * 参数：

    * fd: 文件描述符

    * st: 文件状态结构体

      ```
      struct Stat {
      	uint64 dev,		// 文件所在磁盘驱动器号，不考虑 
      	uint64 ino,		// inode 文件所在 inode 编号
      	uint32 mode,	// 文件类型
      	uint32 nlink,	// 硬链接数量，初始为1
      	uint64 pad[7],	// 无需考虑，为了兼容性设计
      }
      
      // 文件类型只需要考虑:
      ＃define DIR 0o040000		// directory
      ＃define FILE 0o100000		// ordinary regular file
      ```

    * 返回值：如果出现了错误则返回 -1，否则返回 0。

    * 可能的错误

      * fd 无效。
      * st 地址非法。