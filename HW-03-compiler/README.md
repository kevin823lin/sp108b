# Compiler

## 語法

```
PROG = STMTS
BLOCK = { STMTS }
STMTS = STMT*
STMT = WHILE | BLOCK | ASSIGN
WHILE = while (E) STMT
IF = if (E) STMT (else STMT)?
ASSIGN = id '=' E;
E = F (op E)*
F = (E) | Number | Id
```

## 執行結果

```
PS D:\檔案\課程\1082\系統程式\sp108b\HW-03-compiler> make
gcc -std=c99 -O0 lexer.c compiler.c main.c -o compiler

PS D:\檔案\課程\1082\系統程式\sp108b\HW-03-compiler> .\compiler .\test\if.c
a = 0;
b = 2;
if (a > 0) c = a;
else c = b;      
========== lex ==============
token=a
token==
token=0
token=;
token=b
token==
token=2
token=;
token=if
token=(
token=a
token=>
token=0
token=)
token=c
token==
token=a
token=;
token=else
token=c
token==
token=b
token=;
========== dump ==============
0:a
1:=
2:0
3:;
4:b
5:=
6:2
7:;
8:if
9:(
10:a
11:>
12:0
13:)
14:c
15:=
16:a
17:;
18:else
19:c
20:=
21:b
22:;
============ parse =============
t0 = 0
a = t0
t1 = 2
b = t1
t2 = a
t3 = 0
t4 = t2 > t3
if not T4 goto L0
t5 = a
c = t5
goto L1
(L0)
t6 = b
c = t6
(L1)
``` 