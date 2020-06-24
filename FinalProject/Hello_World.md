# 編寫與執行
## 1. 創建項目目錄

```
C:\Users\kevin>mkdir "%USERPROFILE%/projects"

C:\Users\kevin>cd "%USERPROFILE%/projects"​

C:\Users\kevin\projects>mkdir hello_world

C:\Users\kevin\projects>cd hello_world

C:\Users\kevin\projects\hello_world>
```

* 在環境變數 %USERPROFILE% (C:\Users\kevin) 底下創建項目目錄

![](https://github.com/kevin823lin/sp108b/tree/master/FinalProject/src/2.1.png)

## 2. 編寫並執行 Rust 程式

* 建立 Rust 程式：main.rs
    
```
fn main() {

 println!("Hwllo, world!");

}
```
* 輸入`rustc main.rs`來編譯 main.rs
  
* 輸入`main`或下列格式來執行 main.exe
    

![](https://github.com/kevin823lin/sp108b/tree/master/FinalProject/src/2.2.png)

# ​分析程式碼
## 1. main()
main 代表 Rust 程式的進入點
```
fn main() {

}
```
## 2. println!
* Rust 的縮排是 4 個空格，而非'\t'
* `println!`呼叫了一個 Rust 宏 (macor)；呼叫函數時會使用`println`
* `"Hello, world!"`是傳遞給`println!`的字串參數
* 以`;`為結束符號
```
    println!("Hello, world!");
```