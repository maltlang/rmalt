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
'''malt
λ __version__
λ (__version__)
λ +
λ (+)
λ (+ 1 2)
λ (+ +1 -2)
λ (+ -1 1) # 这是语言故意这么设计的，对于不熟悉的人来说就是坑
λ (+ 1.1 -1.2)
'''