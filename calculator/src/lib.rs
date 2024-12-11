/// 定义计算器支持的运算符
#[derive(Debug)]
pub enum Operator {
    Add,      // 加法 +
    Subtract, // 减法 -
    Multiply, // 乘法 *
    Divide,   // 除法 /
}

/// 计算错误类型
#[derive(Debug)]
pub enum CalculatorError {
    DivideByZero,
    InvalidOperator,
    InvalidNumber,
}

/// 解析操作符字符串
pub fn parse_operator(op: &str) -> Result<Operator, CalculatorError> {
    match op {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        _ => Err(CalculatorError::InvalidOperator),
    }
}

/// 执行计算
pub fn calculate(num1: f64, operator: Operator, num2: f64) -> Result<f64, CalculatorError> {
    match operator {
        Operator::Add => Ok(num1 + num2),
        Operator::Subtract => Ok(num1 - num2),
        Operator::Multiply => Ok(num1 * num2),
        Operator::Divide => {
            if num2 == 0.0 {
                Err(CalculatorError::DivideByZero)
            } else {
                Ok(num1 / num2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        assert_eq!(calculate(2.0, Operator::Add, 2.0), Ok(4.0));
        assert_eq!(calculate(5.0, Operator::Subtract, 3.0), Ok(2.0));
        assert_eq!(calculate(4.0, Operator::Multiply, 3.0), Ok(12.0));
        assert_eq!(calculate(10.0, Operator::Divide, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(matches!(
            calculate(5.0, Operator::Divide, 0.0),
            Err(CalculatorError::DivideByZero)
        ));
    }

    #[test]
    fn test_operator_parsing() {
        assert!(matches!(parse_operator("+"), Ok(Operator::Add)));
        assert!(matches!(parse_operator("-"), Ok(Operator::Subtract)));
        assert!(matches!(parse_operator("*"), Ok(Operator::Multiply)));
        assert!(matches!(parse_operator("/"), Ok(Operator::Divide)));
        assert!(matches!(parse_operator("x"), Err(CalculatorError::InvalidOperator)));
    }
} 