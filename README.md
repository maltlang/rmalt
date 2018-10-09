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

***注意：前面不加符号表示无符号整数，加符号表示有符号整数，浮点数除外***

## Demo
malt(malt-repl):
注意：λ是命令提示符
```malt
λ --version--
λ (--version--)
λ +
λ (+)
λ (+ 1 2)
λ (+ +1 -2)
λ (+ -1 1)  # 这是语言故意这么设计的，对于不熟悉的人来说就是坑
λ (+ 1.1 -1.2)
λ # 更新
λ (if true 1 2)
λ (if false 1 2)
λ (if false 1)
```

惊喜
```malt
λ (loop!)   # 一行代码使你的笔记本变成暖手宝
```
