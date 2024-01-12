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
