以下是原書中 Cargo 的介紹
> Cargo 是 Rust 的构建系统和包管理器。大多数 Rustacean 们使用 Cargo 来管理他们的 Rust 项目，因为它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。（我们把代码所需要的库叫做 依赖（dependencies））。
## 1. 查看版本狀態
```
cargo --version
```
## 2. 使用 Cargo 創建項目
```
cargo new hello_cargo
cd hello_cargo
```
* 用 Cargo 建立項目會自動建立 Cargo.toml, src/main.rs 與 git 倉

![](https://github.com/kevin823lin/sp108b/tree/master/FinalProject/src/3.1.png)

## 3. 查看`Cargo.toml`
* `[package]`，一個片段標題，用來定義 package
* `name`，項目名稱
* `version`，項目版本
* `authors`，項目作者
* `edition`，要使用的 Rust 版本
* `[dependencies]`，一個片段標題，列出套件依存項目
```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["kevin823lin <kevin823lin@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
## 4. 查看`src/main.rs`
* Cargo 自動生成一個 Hello, world! 的範例
```
fn main() {
    println!("Hello, world!");
}
```

## 5. 編譯並執行 Cargo 項目
* Cargo 編譯完的檔案會在`.\target\debug\hello_cargo.exe`
```
cargo run
```

![](https://github.com/kevin823lin/sp108b/tree/master/FinalProject/src/3.2.png)

## 6. 單純並快速地檢查程式碼是否能編譯
```
cargo check
```

![](https://github.com/kevin823lin/sp108b/tree/master/FinalProject/src/3.3.png)