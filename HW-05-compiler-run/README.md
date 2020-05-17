# 習題: 為 05-compiler-run 加上 IF 的語法

## 檔案關係
```
main.c => lexer.c
          compiler.c
          ir.c       => irvm.c
```

## 參數
- -ir：印出中間碼
- -run：執行


## 測試方法
1. 於作業目錄執行make以透過Makefile寫好的參數編譯compiler
2. 執行.\compiler .\test\if.c -ir -run
## 測試範例 .\test\if.c
```
a = 3;
b = 5;
if (a > b)
    t = a;
else
    t = b;
```

## 執行結果
```
PS D:\檔案\課程\1082\系統程式\sp108b\HW-05-compiler-run> make
gcc -std=c99 -O0 ir.c irvm.c map.c util.c lexer.c compiler.c main.c -o compiler
PS D:\檔案\課程\1082\系統程式\sp108b\HW-05-compiler-run> .\compiler .\test\if.c -ir -run
=======irDump()==========
00: t1 = 3
01: a = t1
02: t1 = 5
03: b = t1
04: t2 = a
05: t3 = b
06: t4 = t2 > t3
07: ifnot t4 goto L1
08: t1 = a
09: t = t1
10: goto L2
11: (L1)
12: t1 = b
13: t = t1
14: (L2)
===================irRun()=======================
00: t1 = 3 (3)
01: a = t1 (3)
02: t1 = 5 (5)
03: b = t1 (5)
04: t2 = a (3)
05: t3 = b (5)
06: t4 = t2 > t3 (0)
07: ifnot t4 (0) goto L1 (11)
11: (L1) (11)
12: t1 = b (5)
13: t = t1 (5)
14: (L2) (14)
```