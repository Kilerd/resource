 - title = 安装 Rust 及 IDE 的选择
 - url = installation-and-setup
 - datetime = 2020-08-20T23:15:47.187706+08:00
 - template = page.html
 - draw = false


# 安装 Rust
Rust 官方提供了一个在各个平台都极其方便的安装工具 rustup，它提供了以下功能：
 - 安装各个版本的 Rust 及 Cargo (指定版本、stable、beta、nightly)
 - 相关组件(Component)的安装，包括不仅限于 clippy 等
 - 各个平台的 target 编译器

运行以下命令即可完成 rustup 的安装：
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## 更换 rustup 国内源 (可选)
对于国内用户，更换 rustup 源可以带来比较稳定的安装、更新体验。 这里推荐使用中科大 USTC 源

设置环境变量 `RUSTUP_DIST_SERVER` (用于更新 toolchain)
```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
```

以及 `RUSTUP_UPDATE_ROOT` (用于更新 rustup)
```shell
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```
更多详细内容可以参考 [USTC@rustup 实用说明](https://lug.ustc.edu.cn/wiki/mirrors/help/rust-static)

**注：推荐把这两个变量写入 bash/zsh 等工具的配置文件中，以长期生效**

## 安装 nightly 版本
安装好 rustup 之后，nightly 版本的 Rust 安装就变得极其简单。只需运行 `rustup install nightly` 即可。以后的版本更新也可以通过 `rustup update` 完成。

# IDE 的选择和配置

> TIP: 若需要进行小规模代码演示，或者需要把问题代码复现并在社区、论坛上询问。建议使用 [Rust 官方的 Playground](https://play.rust-lang.org/)。
> 
> Rust Playground 可以进行线上执行代码、把代码编译成 MIR 与 汇编 等。可以方便不同的人员对于同一份代码进行分析。


目前来说 Rust 的 IDE 支持度并不是很好，都是处于勉强能使用的阶段，以下会给出几个常见的 IDE 搭配和优劣：

| IDE    | 语法高亮 | Snippets | 代码提示 | Lint | 代码格式化 | 跳转至定义 | 调试 | 文档提示 |
| ------ | -------- | -------- | -------- | ---- | ---------- | ---------- | ---- | -------- |
| IDEA   | ✓        | ✓        | ✓        | ✓    | ✓          | ✓          | ✓    | ✓        |
| vscode | ✓        | ✓        | ✓        | ✓    | ✓          | ✓          | ✓    | ✓        |
| atom   | ✓        | ✓        | ✓        | ✓    | ✓          | ✓          |      | ✓        |
| vim    | ✓        | ✓        | ✓        | ✓    | ✓          | ✓          |      | ✓        |
| emacs  | ✓        | ✓        | ✓        | ✓    | ✓          | ✓          |      | ✓        |

简而言之，常见的 IDE 基本都支持了语法高亮、代码提示、格式化、文档提示等等，唯一的区别在于调试工具的支持。
目前仅有两个支持调试的 IDE 是： 
 - vscode 需要自己配置 LLDB
 - IDEA 也需要自己配置LLDB，基于 IDEA 的 Clion 可以零配置调试

因为对于新手而言，选择 **Clion + intelliJ-Rust** 插件是几乎会省心的组合，如果您需要更佳详细的 IDE 信息，可以查阅 [areweideyet](https://areweideyet.com/)。
