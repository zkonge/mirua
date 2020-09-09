# Mirua

Mirai launcher in Rust

又又又一个Mirai一键启动器

**如需使用mirai-native，请仔细阅读第一次启动时生成的配置文件**

## 特色

+ 自动下载 Mirai 所需要的运行环境
+ 得益于 OpenJ9，平均可减少30%以上的内存占用（由于官方支持原因，32位jre依然使用hotspot）
+ 自定义 Mirai 套件版本号
+ 自定义 Mirai-console 入口点（再也不怕被开发者背刺辣）
+ 下载源来自阿里云，速度++++
+ 自升级
+ ~~全平台支持~~ 摸了

## 下载地址

在下载后，Windows 平台用户请手动在文件名最后添加 `.exe` 扩展名

|系统|下载链接|
|-|-|
|Windows 64位|[mirua_windows_x86_64](https://cdn.jsdelivr.net/gh/zkonge/mirua-update@master/v0.1.2/mirua_windows_x86_64)|
|Linux 64位|[mirua_linux_x86_64](https://cdn.jsdelivr.net/gh/zkonge/mirua-update@master/v0.1.2/mirua_linux_x86_64)|
|MacOS 64位|[mirua_macos_x86_64](https://cdn.jsdelivr.net/gh/zkonge/mirua-update@master/v0.1.2/mirua_macos_x86_64)|

~~别的处理器指令集先摸了~~

欢迎改进 GitHub Action 构建脚本

## 注意

1. Unix系需要系统提前预装 `openssl`（或者类似的玩意）
2. 自动登录功能因为 Mirai-Console 自身的原因暂时没法实现
    > 目前 Windows 思路是向当前控制台窗口发送键盘事件
    > ~~有谁想来试试的吗~~
3. 所以目前自动登录推荐使用 [Pai2Chen/mirai-console-addition](https://github.com/Pai2Chen/mirai-console-addition)
4. 日志等级可由环境变量 `RUST_LOG` 控制，例如 `export RUST_LOG=debug`