use std::io::{self, Write};
use calculator::{calculate, parse_operator, CalculatorError};

fn main() {
    println!("欢迎使用 Rust 计算器！");
    println!("支持的运算：+ (加法), - (减法), * (乘法), / (除法)");
    println!("输入 'q' 退出程序\n");

    loop {
        // 获取第一个数字
        let num1 = match get_number("请输入第一个数字: ") {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 获取运算符
        print!("请输入运算符 (+, -, *, /): ");
        io::stdout().flush().unwrap();
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).unwrap();
        let operator = operator.trim();

        // 检查是否退出
        if operator == "q" {
            println!("感谢使用，再见！");
            break;
        }

        // 解析运算符
        let operator = match parse_operator(operator) {
            Ok(op) => op,
            Err(_) => {
                println!("错误：无效的运算符！");
                continue;
            }
        };

        // 获取第二个数字
        let num2 = match get_number("请输入第二个数字: ") {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 执行计算
        match calculate(num1, operator, num2) {
            Ok(result) => println!("结果: {}\n", result),
            Err(CalculatorError::DivideByZero) => println!("错误：除数不能为零！\n"),
            Err(_) => println!("计算出错！\n"),
        }
    }
}

/// 从用户输入获取数字
fn get_number(prompt: &str) -> Result<f64, CalculatorError> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "q" {
        println!("感谢使用，再见！");
        std::process::exit(0);
    }

    input.parse::<f64>().map_err(|_| CalculatorError::InvalidNumber)
}
