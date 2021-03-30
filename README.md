## rCore_tutorial_v3 TESTS

更新通知：make 命令经过优化，使用格式改为　`make all CHAPTER=x` 可获得第 x 章的测例。

- 可选项 2, 2_bad, 3_0, 3_1, 3_2, 4, 5, 6, 7。

**重要**-加载地址更新：

- chapter2 所有程序加载位置位于 0x80400000，与示例代码一致。
- chapter3 测试程序分为 3 批，每一批的地址都为 0x80400000 + id\*0x20000，id 为程序在这一批中的序号。每一批都与参考代码一致，请分别测试。
- chapter4-7 所有程序加载位置位于 0x0，与示例代码一致。

lab5 默认加载程序的说明：

由于测例中没有 initproc 程序，大家在测试的时候可以手动替换为 ch5_usershell / ch5_usertest 等。最终提交时保持加载 initproc 即可，CI 测试中会进行名称替换。

> `Makefile` 中会根据章节将 `chx_usertest.elf` 复制到 `initproc.elf`。如有需要，可以自行删除并用其他程序作为 `initproc`

可以在 `user/build/asm` 目录下查看汇编来确认加载地址。

其他内容详见 [guide](./guide.md) 。
