# 条件结构

请看如下的猜数字游戏：

```python
import random

print("==猜数字(0-20)==")
number = random.randint(0,20)
while True:
    guess = int(input("请输入你猜的数字："))
    if guess > number:
        print("猜大了...")
        continue
    elif guess < number:
        print("猜小了...")
        continue
    else:
        print("猜对了！")
        break
print("==猜数字游戏结束==")
```

程序运行结果：

![Python猜数字游戏](https://sophia-1303119720.cos.ap-nanjing.myqcloud.com/Python%E7%8C%9C%E6%95%B0%E5%AD%97%E6%B8%B8%E6%88%8F.png)

:bulb: 程序解释：


- `int(input("请输入你猜的数字："))`：`input()` 表示输出一段提示语句，然后以字符串形式接收你输入的内容，`int()` 表示把输入的内容转化为整数。

- 从以上程序可以看出 Python 程序块没有 `{}`  ，而是以  `:`  开始的**缩进组织**方式。

如果不缩进：

```python
while True:
guess = int(input("请输入你猜的数字："))
```
会报错：
```python
File "<string>", line 6
    guess = int(input("请输入你猜的数字："))
    ^
IndentationError: expected an indented block
```

如果缩进格式不统一：

```python
import random

print("==猜数字(0-20)==")
number = random.randint(0,20)
while True:
    guess = int(input("请输入你猜的数字："))
  if guess > number:
        print("猜大了...")
```

也会报错：

```python
File "<string>", line 7
if guess > number:
^
IndentationError: unindent does not match any outer indentation level
```
Python 是有意这么设计的，以提高程序的可读性。
- Python 没有 `swith ... case...`语句：[设计和历史常见问题 — Python 3.9.7 文档](https://docs.python.org/zh-cn/3/faq/design.html#why-isn-t-there-a-switch-or-case-statement-in-python)



# 循环

循环有两种方式：

1. `whie...:`
2. `for ... in ...:`

九九乘法表：

```python
x=1
while x<=9:
    y=1
    while y<=x:
        print(f"{x:<2}*{y:<2}={x*y:<3}", end="")
        y=y+1
    print("")
    x=x+1
```

- Python 块代码采用**缩进**的方式组织。
- `print()` 可以通过参数 `end`  控制是否换行，默认是 `\n` 。

请看下面数羊代码：

```python
import time

sheep_count = int(input("你有多少只羊:"))
for number in range(1, sheep_count+1):
    print(f"{number} 只羊")
    time.sleep(1)
```

- Python 的 `for` 语句与 C++ 不同，它主要作用于序列或迭代器，`rang()` 是一个迭代器，可以生成等差序列：

```python
for x in range(0, 20, 2):
    print(x, end=" ")
```

三个参数代表的含义是最小值，最大值，步长，并且遵循“左闭右开”的惯例。

中午吃什么：

```python
lunches = ['肉夹馍','火锅','刀削面','炒年糕','麻辣香锅','烤肉饭']

for lunch in lunches:
    print(f"吃 {lunch}")
```



## 两个循环控制关键字

`continue` 与 `break` 是两个控制循序执行流程的关键字，具体功能如图所示：

![continue_break](https://sophia-1303119720.cos.ap-nanjing.myqcloud.com/continue_break.png)

与其他语言类似。

