use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub student_id: String,     // 学生学号
    pub subject: String,        // 科目
    pub score: f32,            // 分数
    pub semester: String,       // 学期
}

impl Grade {
    // 创建新成绩
    pub fn new(student_id: String, subject: String, score: f32, semester: String) -> Self {
        Grade {
            student_id,
            subject,
            score,
            semester,
        }
    }

    // 更新成绩
    pub fn update_score(&mut self, new_score: f32) {
        self.score = new_score;
    }

    // 获取成绩等级
    pub fn get_grade_level(&self) -> &str {
        match self.score {
            score if score >= 90.0 => "A",
            score if score >= 80.0 => "B",
            score if score >= 70.0 => "C",
            score if score >= 60.0 => "D",
            _ => "F",
        }
    }

    // 判断是否及格
    pub fn is_passing(&self) -> bool {
        self.score >= 60.0
    }
}

// 实现显示特征
impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "学号: {}, 科目: {}, 成绩: {:.1}, 学期: {}, 等级: {}",
            self.student_id, self.subject, self.score, self.semester, self.get_grade_level()
        )
    }
}

// 为了方便比较和排序，实现 PartialEq
impl PartialEq for Grade {
    fn eq(&self, other: &Self) -> bool {
        self.student_id == other.student_id && 
        self.subject == other.subject && 
        self.semester == other.semester
    }
}

impl Eq for Grade {}
