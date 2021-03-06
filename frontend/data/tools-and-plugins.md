 - title = 实用工具与插件
 - url = tools-and-plugins
 - datetime = 2020-08-20T23:16:54.791307+08:00
 - template = page.html
 - draw = false


- Rust 搜索插件([官网](https://rust-search-extension.now.sh/), [Install to Chrome](https://chrome.google.com/webstore/detail/rust-search-extension/ennpfpdlaclocpomkiablnmbppdnlhoh), [Install to FireFox](https://addons.mozilla.org/firefox/addon/rust-search-extension/)) 在浏览器中快速方便进行 Rust 相关内容搜索：
  - 标准库文档
  - Rust 内部属性(builtin attributes)
  - Error Codes
  - top 20K crates

# awesome cargo
Cargo 作为 Rust 的包管理器，其强大的扩展能力让社区开发出了一系列官方没有的，并且十分实用的插件：
 - [cargo-watch](https://github.com/passcod/cargo-watch) 监听项目的更改并执行命令，极其方便的一个命令，通常用来监听代码更改然后自动执行测试或者重启服务
    - `cargo watch -x test`
    - `cargo watch -x run`
 - [cargo-edit](https://github.com/killercup/cargo-edit) 摆脱手动添加依赖的烦恼，其提供了新增、删除、更新依赖的一系列功能
    - `cargo add` 添加依赖
    - `cargo rm` 删除依赖
    - `cargo upgrade` 更新依赖到最新的版本
 - [cargo-release](https://github.com/sunng87/cargo-release) 用于发布程序的版本，避免手动管理程序版本号，基于 semver 版本管理
 - [cargo-make](https://github.com/sagiegurari/cargo-make) 跟 makefile 类似的机制，当项目庞大，执行命令过多时，可以采用 cargo-make 管理。
 - [cargo-audit](https://github.com/RustSec/cargo-audit) 诊断项目依赖是否具有安全性问题
 - [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) 看看到底是什么导致程序的大小变大
 - [cargo-tree](https://github.com/sfackler/cargo-tree) 显示项目依赖树形图，可以看到次级依赖
 - [cargo-deps](https://github.com/m-cat/cargo-deps) 把项目的依赖图打印出图片的形式导出

这里列出了部分最常用的插件，更加详细全面的内容可以前往[Third party cargo subcommands](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands)

# 其他工具
- [Rust cheet sheet](https://upsuper.github.io/rust-cheatsheet/) 一份对于新手超级友好的 cheet， 可以查看到不同常用结构体的方法，以及相互转换的方式
- [Macro syntax diagram generator](https://lukaslueg.github.io/macro_railroad_wasm_demo/) 一个把 `macro_rules!()` 转换成可视图的在线工具，对于编写宏很有帮助 
- [JSON to struct](https://transform.tools/json-to-rust-serde) 如果需要把已有 JSON 转换成 struct 以序列化时(例如爬虫场景)，可以考虑实用这个工具意见生成 struct