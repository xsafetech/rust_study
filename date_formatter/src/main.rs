use chrono::{NaiveDateTime, NaiveDate, Datelike, Timelike};
use regex::Regex;
use thiserror::Error;
use std::str::FromStr;

#[derive(Error, Debug)]
pub enum DateError {
    #[error("无效的日期格式")]
    InvalidFormat,
    #[error("日期解析错误: {0}")]
    ParseError(String),
    #[error("不支持的格式")]
    UnsupportedFormat,
}

#[derive(Debug)]
pub struct DateFormatter {
    date: NaiveDateTime,
}

impl DateFormatter {
    // 从字符串创建 DateFormatter
    pub fn from_str(date_str: &str) -> Result<Self, DateError> {
        // 尝试不同的日期格式
        if let Ok(date) = Self::parse_standard_format(date_str) {
            return Ok(date);
        }
        if let Ok(date) = Self::parse_chinese_format(date_str) {
            return Ok(date);
        }
        if let Ok(date) = Self::parse_slash_format(date_str) {
            return Ok(date);
        }
        if let Ok(date) = Self::parse_relative_format(date_str) {
            return Ok(date);
        }

        Err(DateError::InvalidFormat)
    }

    // 解析标准格式 (yyyy-MM-dd HH:mm:ss)
    fn parse_standard_format(date_str: &str) -> Result<Self, DateError> {
        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d",
        ];

        for format in formats.iter() {
            if let Ok(date) = NaiveDateTime::parse_from_str(date_str, format) {
                return Ok(DateFormatter { date });
            }
            // 如果只有日期部分，添加时间
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(DateFormatter {
                    date: date.and_hms_opt(0, 0, 0).unwrap(),
                });
            }
        }
        Err(DateError::ParseError("标准格式解析失败".to_string()))
    }

    // 解析中文格式 (2023年12月31日)
    fn parse_chinese_format(date_str: &str) -> Result<Self, DateError> {
        let re = Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日").unwrap();
        if let Some(caps) = re.captures(date_str) {
            let year = caps[1].parse::<i32>().unwrap();
            let month = caps[2].parse::<u32>().unwrap();
            let day = caps[3].parse::<u32>().unwrap();

            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                return Ok(DateFormatter {
                    date: date.and_hms_opt(0, 0, 0).unwrap(),
                });
            }
        }
        Err(DateError::ParseError("中文格式解析失败".to_string()))
    }

    // 解析斜杠格式 (MM/dd/yyyy 或 dd/MM/yyyy)
    fn parse_slash_format(date_str: &str) -> Result<Self, DateError> {
        let formats = [
            "%m/%d/%Y",
            "%d/%m/%Y",
        ];

        for format in formats.iter() {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(DateFormatter {
                    date: date.and_hms_opt(0, 0, 0).unwrap(),
                });
            }
        }
        Err(DateError::ParseError("斜杠格式解析失败".to_string()))
    }

    // 解析相对格式 (today, yesterday, tomorrow)
    fn parse_relative_format(date_str: &str) -> Result<Self, DateError> {
        let today = chrono::Local::now().naive_local().date();
        
        match date_str.to_lowercase().as_str() {
            "today" | "今天" => Ok(DateFormatter {
                date: today.and_hms_opt(0, 0, 0).unwrap(),
            }),
            "yesterday" | "昨天" => Ok(DateFormatter {
                date: today.pred().and_hms_opt(0, 0, 0).unwrap(),
            }),
            "tomorrow" | "明天" => Ok(DateFormatter {
                date: today.succ().and_hms_opt(0, 0, 0).unwrap(),
            }),
            _ => Err(DateError::ParseError("相对格式解析失败".to_string())),
        }
    }

    // 格式化输出
    pub fn format(&self, format: &str) -> Result<String, DateError> {
        match format {
            "standard" => Ok(self.date.format("%Y-%m-%d %H:%M:%S").to_string()),
            "date" => Ok(self.date.format("%Y-%m-%d").to_string()),
            "chinese" => Ok(format!(
                "{}年{}月{}日",
                self.date.year(),
                self.date.month(),
                self.date.day()
            )),
            "slash" => Ok(self.date.format("%m/%d/%Y").to_string()),
            "rfc3339" => Ok(self.date.format("%Y-%m-%dT%H:%M:%S").to_string()),
            _ => Err(DateError::UnsupportedFormat),
        }
    }

    // 获取星期几
    pub fn weekday(&self) -> String {
        let weekdays = ["星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"];
        weekdays[self.date.weekday().num_days_from_sunday() as usize].to_string()
    }

    // 是否是周末
    pub fn is_weekend(&self) -> bool {
        matches!(self.date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
    }

    // 是否是闰年
    pub fn is_leap_year(&self) -> bool {
        let year = self.date.year();
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

fn main() {
    // 测试不同格式的日期
    let test_dates = [
        "2023-12-31",
        "2023年12月31日",
        "12/31/2023",
        "31/12/2023",
        "today",
        "tomorrow",
        "yesterday",
        "今天",
        "明天",
        "昨天",
    ];

    for date_str in test_dates.iter() {
        println!("\n处理日期: {}", date_str);
        match DateFormatter::from_str(date_str) {
            Ok(formatter) => {
                println!("标准格式: {}", formatter.format("standard").unwrap());
                println!("日期格式: {}", formatter.format("date").unwrap());
                println!("中文格式: {}", formatter.format("chinese").unwrap());
                println!("斜杠格式: {}", formatter.format("slash").unwrap());
                println!("RFC3339格式: {}", formatter.format("rfc3339").unwrap());
                println!("星期: {}", formatter.weekday());
                println!("是否周末: {}", formatter.is_weekend());
                println!("是否闰年: {}", formatter.is_leap_year());
            }
            Err(e) => println!("错误: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_format() {
        let formatter = DateFormatter::from_str("2023-12-31").unwrap();
        assert_eq!(formatter.format("standard").unwrap(), "2023-12-31 00:00:00");
    }

    #[test]
    fn test_chinese_format() {
        let formatter = DateFormatter::from_str("2023年12月31日").unwrap();
        assert_eq!(formatter.format("chinese").unwrap(), "2023年12月31日");
    }

    #[test]
    fn test_slash_format() {
        let formatter = DateFormatter::from_str("12/31/2023").unwrap();
        assert_eq!(formatter.format("slash").unwrap(), "12/31/2023");
    }

    #[test]
    fn test_relative_format() {
        let today = chrono::Local::now().naive_local().date();
        let formatter = DateFormatter::from_str("today").unwrap();
        assert_eq!(
            formatter.format("date").unwrap(),
            today.format("%Y-%m-%d").to_string()
        );
    }

    #[test]
    fn test_weekday() {
        let formatter = DateFormatter::from_str("2023-12-31").unwrap();
        assert_eq!(formatter.weekday(), "星期日");
    }

    #[test]
    fn test_is_weekend() {
        let formatter = DateFormatter::from_str("2023-12-31").unwrap();
        assert!(formatter.is_weekend());
    }

    #[test]
    fn test_is_leap_year() {
        let formatter = DateFormatter::from_str("2024-01-01").unwrap();
        assert!(formatter.is_leap_year());
    }
}
