# Malt (rMalt)
The rust implementation of malt programming language

## Parser
Parser is malt parser(default parser)

## BaseType
- Nil
- Bool
- Int
- UInt
- Float
- Symbol
- String
- Tuple
- Dict
- Object
- Function
- Module

***注意：前面不加符号表示无符号整数，加符号表示有符号整数，浮点数除外***

## Demo
**repl没有认真写，所以只能读一行（Parser是能做多行的，但我输入直接用的.read_line）**

malt(malt-repl):

***λ是命令提示符***
```malt
λ add
λ (add)
λ (add 1 2)
λ (add +1 -2)
λ (add -1 1)  # 这是语言故意这么设计的，对于不熟悉的人来说就是坑
λ (add 1.1 -1.2)
λ # 更新
λ (if true 1 2)
λ (if false 1 2)
λ (if false 1)
λ (cond [false 2])
λ (cond [false 2] [true 3])
λ (cond [false 2] [true 3])
λ (loop!)   # 让你的笔记本变成暖手宝
λ # ->_->
λ (lambda [a] a)
λ ((lambda [a] a))
λ (fun T [a b] a)
λ (fun F [a b] b)
λ (fun A [a b] (a b F))
λ (fun O [a b] (a T b))
λ (fun N [v] (v F T))
λ (exit!)
```
