 - title = 各种教程和文章
 - url = tutorials
 - datetime = 2020-08-20T23:16:59.646761+08:00
 - template = page.html
 - draw = false

如果你有相关的教程和文章希望推荐给他人，可以往[这里](https://github.com/Kilerd/resource/blob/master/page/tutorials.md)提出PR。
# Rustlings
一份 JetBrains 根据 [github/rust-lang/rustlings](https://github.com/rust-lang/rustlings) 制作的 Rust 习题，提供了基于知识点的练习题，形式十分相似与 [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html):
 - 你可以下载 [IDEA Edu](https://www.jetbrains.com/education/download/#section=idea) 进行练习
 - 你也可以在 Clion 或者 IDEA Ultimate 中下载 [EduTools Plugin](https://www.jetbrains.com/help/education/install-edutools-plugin.html?section=CLion) 进行学习

更加具体的指导可以查阅[Learner Start Guide - JetBrains](https://www.jetbrains.com/help/education/learner-start-guide.html?section=Rustlings)


# Jon Gjengset 的 Youtube 频道
一位东半球很厉害的 Rust 开发者在 Youtube 上面讲解底层技术的实现和原理，每个视频都涉及到细节。更多他的视频可以点击[这里跳转到 YouTube](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ/videos)。他的视频普遍比较长，通常都是复原了整个开发的过程，包括了 Debug 的阶段。
 - [Porting flamegraph to Rust — part 1](https://www.youtube.com/watch?v=jTpK-bNZiA4)
 - [Porting flamegraph to Rust — part 2](https://www.youtube.com/watch?v=Qy1tQesXc7k)
 - [The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k)
 - [Implementing TCP in Rust (part 1)](https://www.youtube.com/watch?v=bzja9fQWzdA)
 - [Implementing TCP in Rust (part 2)](https://www.youtube.com/watch?v=OCpt1I0MWXE)
 - [Implementing TCP in Rust (part 3)](https://www.youtube.com/watch?v=8GE6ltLRJA4)
 - [Procedural Macros in Rust (part 1)](https://www.youtube.com/watch?v=geovSK3wMB8)
 - [Procedural Macros in Rust (part 2)](https://www.youtube.com/watch?v=KVWHT1TAirU)

# Learn Rust the Dangerous Way
作者从 C 语言使用者的角度讲述了 Rust 的相关知识点

> LRtDW is a series of articles putting Rust features in context for low-level C programmers who maybe don't have a formal CS background — the sort of people who work on firmware, game engines, OS kernels, and the like. Basically, people like me.

- [Why Learn Rust the Dangerous Way?](http://cliffle.com/p/dangerust/0/) Introduction and ground rules.
- [You can't write C in just any ol' language:](http://cliffle.com/p/dangerust/1/) translating a grungy optimized C program into grungy optimized unsafe Rust.
- [References available upon request:](http://cliffle.com/p/dangerust/2/) how Rust references are different from pointers, how they are the same, and why we care.
- [Measure what you optimize:](http://cliffle.com/p/dangerust/3/) taking a hard look at an optimization based on uninitialized memory, and converting it to safe code that's just as fast.
- [A more perfect union:](http://cliffle.com/p/dangerust/4/) considering alternatives to pointer casting, and how to write safe wrappers for unsafe operations.
- [Making safe things from unsafe parts:](http://cliffle.com/p/dangerust/5/) finally converting most of the program to safe code, and making sure that the unsafe bits are safe-ish.
- [Let the compiler do the work:](http://cliffle.com/p/dangerust/6/) a bonus section that looks at how we'd write the program idiomatically in native Rust, and rely on auto-vectorization to make it fast.