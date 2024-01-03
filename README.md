# rs-hecto-practice

这是一个 rust 的练习项目，相关资料为：[Hecto: Build your own text editor in Rust](https://www.flenker.blog/hecto/)。感谢 Flenker 的 blog，我受益匪浅。

This is an exercise project for Rust, and the relevant materials are: [Hecto: Build your own text editor in Rust](https://www.flenker.blog/hecto/). Thank you to Flanker's blog, I have benefited a lot.

## 使用

**注意**

为防止快捷键冲突，请在当前项目路径下使用系统终端启动

**打开**

```bash
# 开启一个新的文件
cargo run

# 打开一个文件
cargo run XXX
```

**操作**

`ctrl + q` 退出，当文件未保存时，需要触发三次以确认不保存退出

`ctrl + s` 保存，当文件为新文件时，需要输入文件名

`ctrl + f` 搜索

## todoList

- TODO 整个项目添加函数注释、关键行注释
- TODO 排查 mac 系统下高亮异常的问题
- TODO 排查无法打开 `.rs` 文件的问题