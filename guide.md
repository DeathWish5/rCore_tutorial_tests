# OS实验的测试要求(v0.1)

### 概述

实现指导见 rcore_tutorial_book_v3，该文档仅解释测试程序。在实现文件系统之前，你应当以合适的方式（可参考知道书或者样例实现）将测试文件置于内存中某一个位置并设法运行。

目前尚无较好的测试脚本，那位大佬有时间请帮忙搞搞 orz。

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
    * C 接口：`int write(int fd, char *buf, int len)`;
    * Rust 接口：`fn write(fd: i32, buf: *mut u8, len: i32) -> i32`;
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
    * C 接口：`int exit(int status);`
    * Rust 接口：`fn exit(status: i32) -> i32;`
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

##### 实现

* `sys_gettime`
  * 功能描述：与 posix `sys_gettime` 不一致　//TODO：改为一致
    * syscall ID：169
    * 功能：获取当前时间。
    * C 接口：long long* time　gettime();
    * Rust 接口：fn gettime() -> i64;
    * 参数：无
    * 返回值：当前时间，单位为毫秒。
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
    * `ch3_2_power`:  测试时需要以批处理形式并发运行`ch3_2_power[012]`，本质就是耗时运算强行拖到时钟中断，正确输出为交替出现的 `power[357] ...`。为了排除其他程序干扰，推荐不要运行除此之外的程序。
    * `ch3_t_deadloop`: 就是一个死循环，只有实现了时钟中断才能把它杀死。
* stride 调度
  * 功能描述：请认真阅读 [ucore doc](https://learningos.github.io/ucore_os_webdocs/lab6/lab6_3_6_1_basic_method.html) 中 stride 调度算法部分，并在自己的 os 中实现该调度。
  * 测试说明：
    * `ch3_3_stride`: 测试时需要以批处理形式并发运行`ch3_3_stride[012345]`，程序本质是过固定时间计数+1，最终输出的计数要和优先级基本成正比。

