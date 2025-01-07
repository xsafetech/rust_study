use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: String,         // 学号
    pub name: String,       // 姓名
    pub class: String,      // 班级
    pub major: String,      // 专业
}

impl Student {
    // 创建新学生
    pub fn new(id: String, name: String, class: String, major: String) -> Self {
        Student {
            id,
            name,
            class,
            major,
        }
    }

    // 更新学生信息
    pub fn update(&mut self, name: Option<String>, class: Option<String>, major: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(class) = class {
            self.class = class;
        }
        if let Some(major) = major {
            self.major = major;
        }
    }
}

// 实现显示特征
impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "学号: {}, 姓名: {}, 班级: {}, 专业: {}",
            self.id, self.name, self.class, self.major
        )
    }
}

// 为了方便比较和排序，实现 PartialEq 和 Eq
impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Student {}
