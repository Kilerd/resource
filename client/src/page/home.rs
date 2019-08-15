use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;
use seed::window;

#[derive(Default)]
pub struct Model {}

#[derive(Clone)]
pub enum Msg {}

impl Model {
    pub fn view(&self) -> Node<Msg> {
        web_sys::window()
            .expect("cannot find the window")
            .document()
            .expect("Can't find the window's document")
            .set_title("Resource.rs - 旨在提供一站式的 Rust 学习资源");
        div![
            img![
                class!["bg"],
                attrs! {At::Src => "/public/rust.png", At::Alt=> "rust logo background"}
            ],
            section![ class!["content"],
                header![
                    img![
                        class!["logo"],
                        attrs! {At::Src => "/public/logo.png", At::Alt=> "resource.rs logo"}
                    ],
                    section![ class!["nav"],
                        h1!["Resource.rs"],
                        p![ class!["description"], "旨在提供一站式的 Rust 学习资源" ],
                        nav![
                            a![ attrs!{At::Href => "/"}, "首页"],
                            a![ attrs!{At::Href => "/posts"}, "最新文章"],
//                            a![ attrs!{At::Href => "https://intellij-rust.github.io/"}, "相关博客"],
                            a![ attrs!{At::Href => "/about"}, "关于我们"],
                        ]
                    ],
                ],
                section![ class!["center"],
                    section![ class!["part"],
                        h2!["Getting started"],
                        p![ class!["description"],
                            "跟随这几个链接，你可以的到一个比较完善的 Rust 编写环境。"
                        ],
                        nav! [
                            li![ a![ attrs!{At::Href => "https://rustup.rs/"}, "Rustup"], "官网指定唯一推荐 Rust 安装方式"],
                            li![ a![ attrs!{At::Href => "https://www.rust-lang.org/"}, "Rust Language"], "Rust 语言官网"]
                        ],

                        section! [ class!["part"],
                            h3!["IDEs"],
                            p![ class!["description"],
                                "目前 Rust 的 IDE 还不算十分完善，但是还是有几个选择可以满足日常开发使用的。更多的选择可以查看",
                                a![attrs!{At::Href => "https://areweideyet.com/"}, "Are we IDE yet"],
                                " 。 新入门的小伙伴可以考虑使用 Clion 为主编辑器"
                            ],
                            nav! [
                                li![
                                    "Clion / IDEA + ",
                                    a![ attrs!{At::Href => "https://intellij-rust.github.io/"}, "IntelliJ-Rust 插件"],
                                    br![],
                                    "这个编辑器组合适合于日常开发就使用 Jetbrains 全家桶的小伙伴。代码补全和提示完全独立于 RLS。同时还提供了勉强可用的「宏展开」等功能。Clion 默认支持了 LLDB，可以做到免配置调试 Rust，IDEA 暂不支持。"
                                ],
                                li![
                                    "VS Code + ",
                                    a![ attrs!{At::Href => "https://marketplace.visualstudio.com/items?itemName=rust-lang.rust"},"Rust(rls) 插件"],
                                    br![],
                                    "该组合建立在 RLS 的基础上，编辑器的功能由 Rust 开发组提供。但是由于 RLS 目前处于半抛弃状态，",
                                    a![ attrs!{At::Href => "https://github.com/rust-analyzer/rust-analyzer"},"rust-analyzer(下一代 RLS)"],
                                    " 也还没有完善。"
                                ]
                            ],
                        ],
                        section! [ class!["part"],
                            h3!["Links"],
                            p![ class!["description"],
                                "那些日常开发中会经常使用的网站"
                            ],
                            nav! [
                                li![
                                    a![ attrs!{At::Href => "https://doc.rust-lang.org/std/"}, "Standard Library API Reference"],
                                    " 标准库文档"
                                ],
                                li![
                                    a![ attrs!{At::Href => "https://docs.rs/"}, "Docs.rs"],
                                    " 查看第三方库的文档索引"
                                ],
                                li![
                                    a![ attrs!{At::Href => "https://crates.io/"}, "Crates.io - Rust Package Registry"],
                                    " Rust 第三方库仓库"
                                ],
                                li![
                                    a![ attrs!{At::Href => "https://lib.rs/"}, "Lib.rs - Find quality Rust libraries and applications."],
                                    " 一个提供了不同领域下优质 crate 的索引。当你需要查看某个领域比较流行的 crate 时，可以阅读。"
                                ],
                                li![
                                    a![ attrs!{At::Href => "https://lib.rs/"}, "Lib.rs - Find quality Rust libraries and applications."],
                                    " 一个提供了不同领域下优质 crate 的索引。当你需要查看某个领域比较流行的 crate 时，可以阅读。"
                                ],
                                li![
                                    a![ attrs!{At::Href => "https://upsuper.github.io/rust-cheatsheet/"}, "Rust cheat sheet"],
                                    " 一份由 @Upsuper 大佬实现的关于 Option、Result、Iterator 和 [u8] 的相关函数速查表，相当实用。"
                                ],

                            ],
                        ]
                    ],

                    section![ class!["part"],
                        h2!["Books"],
                        p![ class!["description"],
                            "那些 Rust 相关的书籍"
                        ],
                        nav![
                            li![
                                "「The Rust Programming Language」",
                                "(",
                                a![ attrs!{At::Href => "https://doc.rust-lang.org/book/#the-rust-programming-language"}, "English"],
                                " / ",
                                a![ attrs!{At::Href => "https://kaisery.github.io/trpl-zh-cn/foreword.html"}, "中文版"],
                                ")",
                                br![],
                                "这是由官方提供的面向全体 Rust 人员的教程，简称「TRPL」。非常适合入门的小伙伴阅读，因为该书系统得提供了 Rust 的特点和设计思想。该书也可以当作 Cookbook，在碰到问题时反复查看。"
                            ],
                            li![
                                "「Rust by Example」",
                                "(",
                                a![ attrs!{At::Href => "https://doc.rust-lang.org/rust-by-example/"}, "English"],
                                ")",
                                br![],
                                "该书在设计上很像 TRPL，但是提供了很多适合用于实践的例子，以体验 Rust 的设计思想。该书同样适合于入门，但是原理和内容不及 TRPL 丰富，但是通过项目驱动可以得到较多的练习过程。"
                            ],
                            li![
                                "「The Little Book of Rust Macros」/「宏小本」",
                                "(",
                                a![ attrs!{At::Href => "https://danielkeep.github.io/tlborm/book/index.html"}, "English"],
                                " / ",
                                a![ attrs!{At::Href => "http://blog.luxko.site/tlborm-chinese/book/index.html"}, "中文版"],
                                ")",
                                br![],
                                "开发者们都说 Rust 的宏是卫生宏，比CPP的宏如何优秀如何好用。那么这本小册子就从源头讲解了 Rust 的整个宏系统，但是由于编写时间过早并没有讲解过程宏。"
                            ],
                            li![
                                "「Asynchronous Programming in Rust」/「Rust 中的异步编程」",
                                "(",
                                a![ attrs!{At::Href => "https://rust-lang.github.io/async-book/"}, "English"],
                                " / ",
                                a![ attrs!{At::Href => "https://huangjj27.github.io/async-book/index.html"}, "中文版"],
                                ")",
                                br![],
                                "鉴于 Future 和 async/awiat 语法的基本实现完成，Rust 中新一代的异步系统已经达成了 MVP 的目标，官方便开始逐步编写和完善该书。该书阐述了 Future 的生态和实现，比较详细地描述了 executor 和 waker 的工作机制。",
                                b!["建议用到异步特性的使用者必读"]
                            ],
                            li![
                                "「Rust Cookbook」",
                                "(",
                                a![ attrs!{At::Href => "https://rust-lang-nursery.github.io/rust-cookbook/intro.html"}, "English"],
                                ")",
                                br![],
                                "Rust Cookbook一本覆盖面很全，例子不是特别多的 cookbook，但是提供了很多常见编程内容的例子和 DEMO，内容包括了算法、CLI编程、解压缩、并发、加解密、数据结构、数据库调用、日期与时间、DEBUG、版本控制、链接C库、编码与解码、文件系统、内存管理、TCP/IP、操作系统操作、数学运算、文本处理、爬虫、URL处理、MIME、HTTP客户端调用 等。"
                            ],
                        ]
                    ],
                    section![ class!["part"],
                        h2![ "推荐阅读" ],
                        p![ class!["description"],
                            "这里只会推荐比较成体系的文章列表，更多更及时的文章可以前往 ",
                            a![ attrs!{At::Href => "/posts"}, "/Posts"],
                            " 页面查看。 "
                        ],
                        section![ class!["part"],
                            h3!["RFCs"],
                            p![ class!["description"],
                                "通过阅读重要的 RFC，可以了解 Rust 的设计理念和发展路线。这里只提供了在 Rust 生涯中比较重要的 RFC，经可能提供译文，全量的内容可以阅读官方",
                                a![ attrs!{At::Href => "https://github.com/rust-lang/rfcs"}, "RFC repo" ],
                                " / ",
                                a![ attrs!{At::Href => "https://rust-lang.github.io/rfcs/introduction.html"}, "Rust RFC Book" ],
                            ],
                            nav![
                                li!["RFC 2592 / Futures"],
                                li!["RFC 1522 / Conservative Impl Trait"],
                                li!["RFC 2349 / Pin"]
                            ]
                        ],
                        section![ class!["part"],
                            h3!["Actix-web 实现的授权微服务应用"],
                            p![ class!["description"],
                                "一份比较完善的 actix-web 实践教程，提供了一份比较容易上手的 actix 写法，同事体验 actor 的强劲性能。不过可能需要读者自己鉴别 acitx-web 0.7 和 1.0 的区别。",
                            ],
                            nav![
                                li![a![ attrs!{At::Href => "https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/"}, "AUTH WEB MICROSERVICE WITH RUST USING ACTIX-WEB - COMPLETE TUTORIAL PART 1"]],
                                li![a![ attrs!{At::Href => "https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-2/"}, "AUTH WEB MICROSERVICE WITH RUST USING ACTIX-WEB - COMPLETE TUTORIAL PART 2"]],
                                li![a![ attrs!{At::Href => "https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-3/"}, "AUTH WEB MICROSERVICE WITH RUST USING ACTIX-WEB - COMPLETE TUTORIAL PART 3"]],
                                li![a![ attrs!{At::Href => "https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/"}, "AUTH WEB MICROSERVICE WITH RUST USING ACTIX-WEB 1.0 - COMPLETE TUTORIAL"]],
                            ]
                        ],
                        section![ class!["part"],
                            h3!["Jon Gjengset 的 Youtube 频道"],
                            p![ class!["description"],
                                "一位东半球很厉害的 Rust 开发者在 Youtube 上面讲解底层技术的实现和原理，每个视频都涉及到细节。更多他的视频可以点击",
                                a![ attrs!{At::Href => "https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ/videos"}, "这里跳转到 YouTube" ],
                                "。同时如果你希望他下一个视频讲解其他内容的话，可以点击",
                                a![ attrs!{At::Href => "https://jon.thesquareplanet.com/live-coding/"}, "Rust live-coding stream" ],
                                "进行投票和参与讨论。"
                            ],
                            nav![
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=jTpK-bNZiA4"}, "Porting flamegraph to Rust — part 1"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=Qy1tQesXc7k"}, "Porting flamegraph to Rust — part 2"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=9_3krAQtD2k"}, "The What and How of Futures and async/await in Rust"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=bzja9fQWzdA"}, "Implementing TCP in Rust (part 1)"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=OCpt1I0MWXE"}, "Implementing TCP in Rust (part 2)"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=8GE6ltLRJA4"}, "Implementing TCP in Rust (part 3)"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=geovSK3wMB8"}, "Procedural Macros in Rust (part 1)"]],
                                li![a![ attrs!{At::Href => "https://www.youtube.com/watch?v=KVWHT1TAirU"}, "Procedural Macros in Rust (part 2)"]],
                            ]
                        ]
                    ]
                ]
            ]
        ]
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    model.view()
}

pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    Model { ..Model::default() }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {}
}
