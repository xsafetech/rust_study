use std::io::{self, Write};
// 需要添加这行来导入 lib.rs 中的函数
use temperature_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// 温度转换程序的主函数
/// 提供交互式界面让用户进行温度转换
fn main() {
    loop {
        print_menu();
        
        // 获取并处理用户选择
        let choice = match get_user_choice() {
            Ok(c) => c,
            Err(e) => {
                println!("错误: {}", e);
                continue;
            }
        };

        // 检查是否退出程序
        if choice == 0 {
            println!("程序已退出");
            break;
        }

        process_conversion(choice);
    }
}

/// 打印程序菜单选项
/// 显示所有可用的转换功能
fn print_menu() {
    println!("\n温度转换程序");
    println!("1. 摄氏度转华氏度 (°C → °F)");
    println!("2. 华氏度转摄氏度 (°F → °C)");
    println!("3. 批量转换（多个摄氏度值）");
    println!("0. 退出程序");
    print!("请选择功能 (0-3): ");
    io::stdout().flush().unwrap();
}

/// 获取用户输入的菜单选项
/// 
/// # 返回值
/// * `Ok(u32)` - 用户选择的有效选项（0-3）
/// * `Err(String)` - 输入错误的描述信息
fn get_user_choice() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "无法读取输入".to_string())?;
    
    input.trim().parse::<u32>()
        .map_err(|_| "请输入有效的数字".to_string())
        .and_then(|num| {
            if num <= 3 {
                Ok(num)
            } else {
                Err("请输入0-3之间的数字".to_string())
            }
        })
}

/// 根据用户选择处理相应的温度转换
/// 
/// # 参数
/// * `choice` - 用户的选择（1: 摄氏转华氏, 2: 华氏转摄氏, 3: 批量转换）
fn process_conversion(choice: u32) {
    match choice {
        1 => single_conversion(true),   // 摄氏度转华氏度
        2 => single_conversion(false),  // 华氏度转摄氏度
        3 => batch_conversion(),        // 批量转换
        _ => println!("无效的选择"),
    }
}

/// 处理单次温度转换
/// 
/// # 参数
/// * `is_celsius_to_fahrenheit` - true表示摄氏转华氏，false表示华氏转摄氏
fn single_conversion(is_celsius_to_fahrenheit: bool) {
    let temp_type = if is_celsius_to_fahrenheit { "摄氏度" } else { "华氏度" };
    print!("请输入{}温度: ", temp_type);
    io::stdout().flush().unwrap();

    if let Ok(temp) = get_temperature() {
        let result = if is_celsius_to_fahrenheit {
            celsius_to_fahrenheit(temp)
        } else {
            fahrenheit_to_celsius(temp)
        };
        
        // 格式化输出转换结果，保留两位小数
        println!("{:.2}°{} = {:.2}°{}", 
            temp,
            if is_celsius_to_fahrenheit { "C" } else { "F" },
            result,
            if is_celsius_to_fahrenheit { "F" } else { "C" }
        );
    }
}

/// 批量转换摄氏度温度
/// 允许用户输入多个摄氏度值，程序将它们全部转换为华氏度
fn batch_conversion() {
    println!("请输入多个摄氏度温度值，用空格分隔:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    
    // 解析输入的多个温度值
    let temperatures: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if temperatures.is_empty() {
        println!("没有有效的输入值");
        return;
    }

    // 显示所有转换结果
    println!("\n转换结果:");
    for temp in temperatures {
        let fahrenheit = celsius_to_fahrenheit(temp);
        println!("{:.2}°C = {:.2}°F", temp, fahrenheit);
    }
}

/// 获取用户输入的温度值
/// 
/// # 返回值
/// * `Ok(f64)` - 解析成功的温度值
/// * `Err(String)` - 输入错误的描述信息
fn get_temperature() -> Result<f64, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "无法读取输入".to_string())?;
    
    input.trim().parse::<f64>()
        .map_err(|_| "请输入有效的数字".to_string())
}
