---
title: python 里的 exec
categories: Coding
tags:
  - python
  - traps
date: 2017-07-02 15:58:01
---
# python 里的 exec

```python
a = 1
exec('a += 1')
print(a)
```
如果要在 python 中动态执行代码，*exec* 是一个可行的选择。对于上面这段代码，可以预想到，输出结果为 2 。但是如果看过 python 的文档的话，会注意到 *exec* 的说明中有[这样一段](https://docs.python.org/3/library/functions.html?highlight=exec#exec)：
> Be aware that the return and yield statements may not be used outside of function definitions even within the context of code passed to the exec() function.

什么状况？不是出来结果了吗？再试试下面这段代码
```python
def func_test():
    a = 1
    exec('a += 1')
    print(a)


func_test()
```
执行之后结果竟然还是 1 ? *exec* 没有执行吗？那之前的结果是怎么来的？仔细再看看文档，会注意到其中有这样一段 (然而就是第二段 = =)。
>In all cases, if the optional parts are omitted, the code is executed in the current scope. If only globals is provided, it must be a dictionary, which will be used for both the global and the local variables. If globals and locals are given, they are used for the global and local variables, respectively. If provided, locals can be any mapping object. Remember that at module level, globals and locals are the same dictionary. If exec gets two separate objects as globals and locals, the code will be executed as if it were embedded in a class definition.

换句话说，当 *exec* (exec(object[, globals[, locals]])) 没有给予可选参数时，它的结果只会在当前作用域有效。当 *exec* 在模块级运行时， *locals* 和 *global* 是相同的字典，这也就是第一段代码生效的原因。但在第二段代码中， *print* 打印的是 *global* 中的 *a* ，*exec* 的结果却在 *locals* 中， 这也就是第二段代码不生效的原因。

那怎么解决这个问题呢？
[python3 cookbook](http://python3-cookbook.readthedocs.io/zh_CN/latest/c09/p23_executing_code_with_local_side_effects.html)
给出了这样一个解决方案：
```
def test():
    a = 13
    loc = locals()
    exec('b = a + 1')
    b = loc['b']
    print(b)


test()
```
即手动把 *locals* 里更改过的变量的值传给 *global* 。

真是个大坑。
