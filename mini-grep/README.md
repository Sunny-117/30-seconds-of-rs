```shell
cargo run xxx xxxx.txt // 可以在xxxx.txt中搜索xxx字符串
```

example: `cargo run abcx poem.txt`

区分大小写

`CASE_INSENSIVE=1 cargo run air poem.txt `

标准输出重定向到文件中：

```shell
cargo run > output.txt
cargo run air poem.txt > output.txt
```
