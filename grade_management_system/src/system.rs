use std::collections::HashMap;
use crate::student::Student;
use crate::grade::Grade;

#[derive(Debug)]
pub struct GradeManagementSystem {
    students: HashMap<String, Student>,  // 学号 -> 学生信息
    grades: Vec<Grade>,                 // 所有成绩记录
}

impl GradeManagementSystem {
    // 创建新的成绩管理系统
    pub fn new() -> Self {
        GradeManagementSystem {
            students: HashMap::new(),
            grades: Vec::new(),
        }
    }

    // 添加学生
    pub fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            return Err(format!("学号 {} 已存在", student.id));
        }
        self.students.insert(student.id.clone(), student);
        Ok(())
    }

    // 删除学生
    pub fn remove_student(&mut self, student_id: &str) -> Result<(), String> {
        if !self.students.contains_key(student_id) {
            return Err(format!("学号 {} 不存在", student_id));
        }
        self.students.remove(student_id);
        // 同时删除该学生的所有成绩记录
        self.grades.retain(|grade| grade.student_id != student_id);
        Ok(())
    }

    // 添加成绩
    pub fn add_grade(&mut self, grade: Grade) -> Result<(), String> {
        if !self.students.contains_key(&grade.student_id) {
            return Err(format!("学号 {} 不存在", grade.student_id));
        }
        // 检查是否已存在相同学期相同科目的成绩
        if self.grades.iter().any(|g| 
            g.student_id == grade.student_id && 
            g.subject == grade.subject && 
            g.semester == grade.semester
        ) {
            return Err(format!(
                "学号 {} 的 {} 学期 {} 科目成绩已存在",
                grade.student_id, grade.semester, grade.subject
            ));
        }
        self.grades.push(grade);
        Ok(())
    }

    // 更新成绩
    pub fn update_grade(&mut self, student_id: &str, subject: &str, semester: &str, new_score: f32) -> Result<(), String> {
        if let Some(grade) = self.grades.iter_mut().find(|g| 
            g.student_id == student_id && 
            g.subject == subject && 
            g.semester == semester
        ) {
            grade.update_score(new_score);
            Ok(())
        } else {
            Err(format!(
                "未找到学号 {} 的 {} 学期 {} 科目成绩",
                student_id, semester, subject
            ))
        }
    }

    // 获取学生信息
    pub fn get_student(&self, student_id: &str) -> Option<&Student> {
        self.students.get(student_id)
    }

    // 获取学生所有成绩
    pub fn get_student_grades(&self, student_id: &str) -> Vec<&Grade> {
        self.grades
            .iter()
            .filter(|grade| grade.student_id == student_id)
            .collect()
    }

    // 获取学生某学期的所有成绩
    pub fn get_student_semester_grades(&self, student_id: &str, semester: &str) -> Vec<&Grade> {
        self.grades
            .iter()
            .filter(|grade| grade.student_id == student_id && grade.semester == semester)
            .collect()
    }

    // 计算学生某学期的平均成绩
    pub fn calculate_semester_average(&self, student_id: &str, semester: &str) -> Option<f32> {
        let semester_grades: Vec<&Grade> = self.get_student_semester_grades(student_id, semester);
        if semester_grades.is_empty() {
            return None;
        }
        let sum: f32 = semester_grades.iter().map(|grade| grade.score).sum();
        Some(sum / semester_grades.len() as f32)
    }

    // 获取所有学生列表
    pub fn get_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }

    // 获取某门课程的所有成绩
    pub fn get_subject_grades(&self, subject: &str, semester: &str) -> Vec<(&Student, &Grade)> {
        self.grades
            .iter()
            .filter(|grade| grade.subject == subject && grade.semester == semester)
            .filter_map(|grade| {
                self.students
                    .get(&grade.student_id)
                    .map(|student| (student, grade))
            })
            .collect()
    }

    // 统计某门课程的成绩分布
    pub fn get_subject_statistics(&self, subject: &str, semester: &str) -> HashMap<String, i32> {
        let mut statistics = HashMap::new();
        statistics.insert("A".to_string(), 0);
        statistics.insert("B".to_string(), 0);
        statistics.insert("C".to_string(), 0);
        statistics.insert("D".to_string(), 0);
        statistics.insert("F".to_string(), 0);

        for grade in self.grades.iter().filter(|g| g.subject == subject && g.semester == semester) {
            let level = grade.get_grade_level().to_string();
            *statistics.get_mut(&level).unwrap() += 1;
        }

        statistics
    }

    // 获取所有成绩列表
    pub fn get_all_grades(&self) -> Vec<&Grade> {
        self.grades.iter().collect()
    }
}
