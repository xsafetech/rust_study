/// 将摄氏度转换为华氏度
// 
/// # 参数
/// * `celsius` - 摄氏度温度值
/// 
/// # 返回值
/// * 返回对应的华氏度温度值
/// * 如果输入无效（非有限数），返回 NaN
/// 
/// # 示例
/// ```
/// use temperature_converter::celsius_to_fahrenheit;
/// assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    if !celsius.is_finite() {
        return f64::NAN;
    }

    // 检查计算结果是否会溢出
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    if fahrenheit.is_finite() {
        fahrenheit
    } else {
        f64::NAN
    }
}  

/// 将华氏度转换为摄氏度
/// 
/// # 参数
/// * `fahrenheit` - 华氏度温度值
/// 
/// # 返回值
/// * 返回对应的摄氏度温度值
/// * 如果输入无效（非有限数），返回 NaN
/// 
/// # 示例
/// ```
/// use temperature_converter::fahrenheit_to_celsius;
/// assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
/// assert_eq!(fahrenheit_to_celsius(212.0), 100.0);

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    if !fahrenheit.is_finite() {
        return f64::NAN;
    }
    // 检查计算结果是否会溢出
    let celsius = (fahrenheit - 32.0) *5.0 / 9.0;
    if celsius.is_finite() {
        celsius
    } else {
        f64::NAN
    }
}