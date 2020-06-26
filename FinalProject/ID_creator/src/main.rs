use std::io;
use rand::Rng;
use std::process;
use std::io::Write;

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

fn main() {

    println!("歡迎使用身分證製造機~"); // 進入程式輸出歡迎字樣

    while(true) {

        print!("請問要製造的身分是(0隨機 1男 2女 ; 輸入 -1 離開程式)？"); // 詢問欲生成的性別
        let _ = io::stdout().flush();

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
