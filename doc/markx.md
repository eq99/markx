# 背景

Markdown 是一个轻量的标记语言，由于它的语法简洁，因此深受程序员喜爱。程序员一般用 Markdown 写文档，写博客，做笔记。

Markdown 语法没有一个统一的标准，因此会出现编辑器解析器语法不兼容的问题，常见的 Markdown 标准是 [CommonMark](https://commonmark.org/) 标准，除此之外还有一些特定平台支持的 Markdown：

- Discourse
- GitHub
- GitLab
- Reddit

CommonMark 语法标准可以参考 [CommonMark Spec](https://spec.commonmark.org/) 。



# Markdown 文档的组成

一篇 Markdown 文章由块级元素组成，例如自然段，代码块，数学公式块，列表，表格，预定义样式块，标题等。块级元素又可以分为两类：

- 内容块：不再包含其他块级元素。例如：代码块，数学公式块，表格，段落。
- 容器块：能包含内容块，但不能包含容器块。例如：预定义样式块。

每个块级元素有特定的内部结构，例如：

- 代码块：可能包含语言，语法高亮，行号等。
- 公式块：latex解析。
- 富文本段：常见的格式。

对于富文本文本段，它由行内元素组成：纯文字，加粗，图片，行内代码，行内公式，链接，Emoji。因此 Markdown 文档的解析可以分为两个步骤：

- 解析文档得到块级元素。
- 解析块级元素。



# 行内元素(Inline)

## 行内代码（Code spans）

由一对反引号括起来的内容：

```markdown
`hi`
```

## Emoji

本解析器不支持Emoji :+1: 语法，请直接复制粘贴相关表情。



## 行内公式（Inline math）

由一对美元符括起来的部分。

```markdown
$e^{i\pi)+1=0$
```

## 自动链接（Autolinks）

由尖括号括起来的文字：

```markdown
<http://foo.bar.baz>
```

会被转为：

```html
<a href="http://foo.bar.baz">http://foo.bar.baz</a>
```



## 加粗（strong emphasis）

由一对双星号括起来的内容：

```shell
**in strong**
```



## 链接（Links）

链接的语法如下所示：

```markdown
[link](/uri "title")
```

会解析成：

```html
<a href="/uri" title="title">link</a>
```



## 图片（Images）

图片语法如下：

```markdown
![foo](url "title")
```

会解析成：

```html
<p><img src="/url" alt="foo" title="title" /></p>
```



## 普通文本(Text)

不是以上行内元素组成的字符串就会被当做普通文本。





# 块级元素

接下来我们来看看一些块级元素的定义，这里定义的块级元素参考了 CommonMark，但并没有完全遵守，我们还自定义了一些块级元素。

块级元素可以分为两种，一种是容器块，一种是内容块。它们之间的区别是容器块可能包含其他块，内容块不包含其他块。



## 标题（ATX headings）

由 1~3 个 `#` 开头，然后是空格，紧接着标题内容，典型示例：

```markdown
# foo
## foo
### foo
```

解析为：

```html
<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
```



## 代码块（Fenced code blocks）

代码块由三个反引号开始，三个反引号结尾，它们不能在同一行。起始单引号后可以跟代码语言名字。

~~~markdown
```ruby
def foo(x)
  return 3
end
```
~~~



## 公式块（Display maths）

代码块由两个美元符开始，由两个美元符结尾，它们不能在同一行。

```markdown
$$
f(x)=1+x+\frac{x^2}{2}
$$
```



## 列表（Lists）

由 `-` 加一个空格开始的连续不断的行。

目前只定义了无序列表。



## 预定义样式（Pres）

预定义样式由两个百分号开始，由两个百分号结尾，它们不能在同一行。起始百分好可以跟预定义样式名：

```markdown
%%warn 这里还可以定义标题
预定义样式里面可能会有如下块级元素

代码块
公式块
列表
自然段

%%
```

预定义的样式名有：

```shell
状态类：
'quote' | '引用'
'warn' | '警告'
'info' | '提示'
'tip' | '技巧'
'think' | '思考

数学类：
'lem' | '定理'
'pro' | '证明'
'def' | '定义'
'thm' | '定理'
'sol' | '解'
'cor' | '推论'
```



## 习题（Exercises）

习题由两个问号开始，由两个问号结尾，它们不能在同一行，起始问号后可以跟习题类型：

```markdown
??ms
这里是主题干。

- 这里的列表定义选项或者小问
- 另一个小问
- 小问之间不能有空行
- 没有小问的用空列表，以便与答案列表区分

- 这里的列表定义相应小问的答案
- 用空行与上面的的列表隔开
??
```

支持的题型有：

- ss：单选题
- ms：多选题
- co：填空题
- qa：问答题





## 自然段（Paragraphs）

 普通的块级元素，不是以上块级元素都被当做自然段。



# 解析算法

第一步：解析块级元素

第二步：对于块级元素，调用相应解析器

