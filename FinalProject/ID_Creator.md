## 第一個小程式-[ID Creator](https://github.com/kevin823lin/sp108b/blob/master/FinalProject/ID_creator/)

## 1. 使用 Cargo 創建項目
```
cargo new ID_creator
cd ID_creator
```

![](https://github.com/kevin823lin/sp108b/blob/master/FinalProject/src/4.1.png?raw=true)

## 2. 開始設計程式

* 預期結果
    1. 進入程式會有歡迎字樣
    2. 詢問是否要指定製造的身分或離開程式
    3. 依照指定的格式輸出身分證字號
    4. 輸入錯誤會報錯

        ```
        歡迎使用身分證製造機~
        請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？0
        Xxxxxxxxxx
        請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？1
        X1xxxxxxxx
        請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？2
        X2xxxxxxxx
        請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？3
        輸入錯誤，請重新輸入
        請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？-1
        掰~

        註：X為英文字母，x為數字
        ```

* 先在`Cargo.toml`中加入`rand = "0.5.5"`

    ```
    [package]
    ...
    [dependencies]
    rand = "0.5.5"
    ```

* 程式分成兩個部分

    1. `create()`函數
    * 依序輸出英文字母、性別碼、數字2~8碼、檢查碼

        ```
        fn create(string: String) {
            let letter = ['A', 'B', 'C', 'D', 
            'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 
            'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 
            'X', 'Y', 'W', 'Z', 'I', 'O']; // 依照驗證代碼順序紀錄英文字母

            let mut sum = 0; // 用 sum 紀錄驗證代碼

            let a = rand::thread_rng().gen_range(0, 3); // 亂數生成英文字母順序第一碼
            sum += a + 1;
            let mut b;
            if (a != 2) {  // 使用亂數生成英文字母順序第二碼
                b = rand::thread_rng().gen_range(0, 10);
            }
            else {
                b = rand::thread_rng().gen_range(0, 6);
            }
            sum += b * 9;

            print!("{}", letter[a * 10 + b]); // 輸出英文字母
            
            
            let mut temp = 0;
            match string.as_str() { // 判斷輸出身分
                "0\r\n" => temp = rand::thread_rng().gen_range(1, 3),
                "1\r\n" => temp = 1,
                "2\r\n" => temp = 2,
                _ => print!(""),
            }
            sum += temp * 8;

            print!("{}", temp);
            
            for i in 1..8 { // 亂數生成數字2~8碼
                let mut temp = rand::thread_rng().gen_range(0, 10);
                sum += temp * (8 - i);

                print!("{}", temp);
            }

            if (sum % 10 == 0) { // 依照規則生成檢查碼
                println!("0");
            }
            else {
                println!("{}", 10 - sum % 10);
            }
        }
        ```
    2. `main()`函數
    * 進入程式會有歡迎字樣
    * 詢問是否要指定製造的身分或離開程式
    * 依照指定的格式輸出身分證字號
    * 輸入錯誤會報錯

        ```
        fn main() {

            println!("歡迎使用身分證製造機~"); // 進入程式輸出歡迎字樣

            while(true) {

                print!("請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？"); // 詢問欲生成的性別

                let mut string = String::new();
                io::stdin().read_line(&mut string).expect("Failed to read line"); // 輸入數字

                match string.as_str() {
                    "-1\r\n" =>  { // 離開程式
                        println!("掰~");
                        process::exit(0);
                    },
                    "0\r\n" | "1\r\n" | "2\r\n" => create(string), // 輸入正確，呼叫 create() 函數
                    _ => { // 輸入錯誤，要求重新輸入
                        println!("輸入錯誤，請重新輸入");
                        continue;
                    },
                }
            }
        }
        ```

## 3. 解決問題
* print! 輸出會在 input 後

    在 main() 函數中，使用print! 提示使用者在該句後輸入數字
    ```
    fn main() {

        println!("歡迎使用身分證製造機~"); // 進入程式輸出歡迎字樣

        while(true) {

            print!("請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？"); // 詢問欲生成的性別

            let mut string = String::new();
            io::stdin().read_line(&mut string).expect("Failed to read line"); // 輸入數字
            
            ...
        }
    }
    ```
    正常應該顯示如下
    ```
    歡迎使用身分證製造機~
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？0 
    L155294445
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？0
    Y275408787
    ```
    實際上卻是不是如此
    ```
    歡迎使用身分證製造機~
    0
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？L155294445
    0
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？Y275408787
    ```
    後來發現問題出在`print!`
    
    只要在`print!`跟`io::stdin()`加入下面這句
    ```
    let _ = io::stdout().flush();
    ```
    像這樣
    ```
    print!("請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？"); // 詢問欲生成的性別
    let _ = io::stdout().flush();

    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line"); // 輸入數字
    ```
    就能正常顯示了
    ```
    歡迎使用身分證製造機~
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？0 
    L155294445
    請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？0
    Y275408787
    ```

# 參考資料
* [中華民國國民身分證 - wiki](https://zh.wikipedia.org/wiki/%E4%B8%AD%E8%8F%AF%E6%B0%91%E5%9C%8B%E5%9C%8B%E6%B0%91%E8%BA%AB%E5%88%86%E8%AD%89#%E7%B7%A8%E8%99%9F%E8%A6%8F%E5%89%87)
* [Why does this read input before printing?
\- Stack Overflow](https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing)