# 30-seconds-of-rs

30 seconds to collect useful rust snippet.

[![License](https://img.shields.io/badge/license-CC0--1.0-blue.svg)](https://github.com/Sunny-117/30-seconds-of-rs/blob/master/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![](https://img.shields.io/github/followers/sunny-117.svg?style=social&label=Follow%20Me)](https://github.com/Sunny-117)

> 你可以在 30 秒或更短时间内收集有用的 rust 代码片段。

- 使用 <kbd>Ctrl</kbd> + <kbd>F</kbd> 或者 <kbd>command</kbd> + <kbd>F</kbd> 来查找代码片段。
- 代码片段基于 Rust，如果你还不熟悉可以在[这里](https://course.rs/about-book.html)学习。

## 目录

### 📚 应用类

<details>
<summary>详细信息</summary>

- [`简易计算器`](#calc)
- [`文件读取器`](#fileReader)

</details>

### calc

输入表达式，格式如 1 + 2 或 3 \* 4，输入 'exit' 退出:

```rust
use std::io;

fn main() {
    loop {
        println!("请输入表达式，格式如 1 + 2 或 3 * 4，输入 'exit' 退出:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        // 去除首尾空格
        let input = input.trim();

        if input == "exit" {
            println!("退出计算器");
            break;
        }

        // 解析表达式
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 3 {
            println!("无效的表达式，请重新输入");
            continue;
        }

        // 获取操作数和运算符
        let operand1: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效的数字: {}", tokens[0]);
                continue;
            }
        };

        let operator = tokens[1];

        let operand2: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效的数字: {}", tokens[2]);
                continue;
            }
        };

        // 执行计算
        let result = match operator {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => {
                if operand2 != 0.0 {
                    operand1 / operand2
                } else {
                    println!("除数不能为零");
                    continue;
                }
            }
            _ => {
                println!("无效的运算符: {}", operator);
                continue;
            }
        };

        // 输出结果
        println!("结果: {}", result);
    }
}

```

### fileReader

```rust
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let content = read_file("assets/demo.csv");
    match content {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
    // 同上功能，语法糖写法：
    let content = read_file("assets/demo.csv")?;
    println!("{}", content);
    Ok(())
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

```

<br>[⬆ 回到顶部](#目录)
