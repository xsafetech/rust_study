use chrono;

// 类型转换器程序的主函数
// 提供多种类型转换功能的演示
fn main() {
    println!("数值类型转换器");

    // 依次演示各种类型转换功能
    basic_type_conversion();
    string_conversion();
    radix_conversion();
    times_conversion();
}

// 基础数值类型之间的转换示例
// 功能
// * 演示整数类型之间的转换 (i32 -> i64)
// * 演示浮点数类型之间的转换 (f64 -> f32)
// 示例
// ```
// basic_type_conversion();
// // 输出:
// // i32 转 i64: 42 -> 42
// // f64 转 f32: 3.14 -> 3.14
// ```
fn basic_type_conversion() {
    // 整数类型转换示例
    let num32: i32 = 42;
    let num64: i64 = num32.into();  // 使用 into() 进行安全转换
    println!("i32 转 i64: {} -> {}", num32, num64);

    // 浮点数转换示例
    let float64: f64 = 3.14;
    let float32: f32 = float64 as f32;  // 使用 as 进行类型转换
    println!("f64 转 f32: {} -> {}", float64, float32);
}

// 字符串与数值类型之间的转换示例
// 功能
// * 演示字符串转整数 (String -> i32)
// * 演示字符串转浮点数 (String -> f64)
// 错误处理
// * 使用 match 语句处理解析失败的情况
fn string_conversion() {
    // 字符串转整数示例
    let int_str = "123";
    match int_str.parse::<i32>() {
        Ok(num) => println!("字符串转整数: {} -> {}", int_str, num),
        Err(e) => println!("转换失败: {}", e),
    }

    // 字符串转浮点数示例
    let float_str = "3.14";
    match float_str.parse::<f64>() {
        Ok(num) => println!("字符串转浮点数: {} -> {}", float_str, num),
        Err(e) => println!("转换失败: {}", e),
    }
}

// 不同进制之间的转换示例
// 功能
// * 将十进制数转换为二进制表示
// * 将十进制数转换为八进制表示
// * 将十进制数转换为十六进制表示
// 格式说明
// * {:b} - 二进制格式
// * {:o} - 八进制格式
// * {:x} - 十六进制格式
fn radix_conversion() {
    let decimal = 42;
    println!("十进制 {} 转换为:", decimal);
    println!("二进制: {:b}", decimal);
    println!("八进制: {:o}", decimal);
    println!("十六进制: {:x}", decimal);
}

// 日期时间格式转换示例
// 功能
// * 获取当前 UTC 时间并格式化输出
// 依赖
// * 使用 chrono 库进行时间处理
fn times_conversion() {
    let now = chrono::Utc::now();
    println!("当前时间: {}", now);
}