use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::student::Student;
use crate::grade::Grade;
use crate::system::GradeManagementSystem;

// 用于序列化的数据结构
#[derive(Serialize, Deserialize)]
struct SystemData {
    students: Vec<Student>,
    grades: Vec<Grade>,
}

impl SystemData {
    // 从系统创建数据结构
    fn from_system(system: &GradeManagementSystem) -> Self {
        SystemData {
            students: system.get_all_students().into_iter().cloned().collect(),
            grades: system.get_all_grades().into_iter().cloned().collect(),
        }
    }

    // 转换为系统
    fn into_system(self) -> GradeManagementSystem {
        let mut system = GradeManagementSystem::new();
        // 先添加所有学生
        for student in self.students {
            let _ = system.add_student(student);
        }
        // 再添加所有成绩
        for grade in self.grades {
            let _ = system.add_grade(grade);
        }
        system
    }
}

pub struct FileIO {
    file_path: String,
}

impl FileIO {
    // 创建新的文件IO实例
    pub fn new(file_path: String) -> Self {
        FileIO { file_path }
    }

    // 保存系统数据到文件
    pub fn save_to_file(&self, system: &GradeManagementSystem) -> Result<(), String> {
        let data = SystemData::from_system(system);
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("序列化数据失败: {}", e))?;
        
        fs::write(&self.file_path, json)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        
        Ok(())
    }

    // 从文件加载系统数据
    pub fn load_from_file(&self) -> Result<GradeManagementSystem, String> {
        if !Path::new(&self.file_path).exists() {
            return Ok(GradeManagementSystem::new());
        }

        let json = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        
        let data: SystemData = serde_json::from_str(&json)
            .map_err(|e| format!("解析数据失败: {}", e))?;
        
        Ok(data.into_system())
    }

    // 导出成绩单到CSV文件
    pub fn export_transcript_to_csv(
        &self,
        system: &GradeManagementSystem,
        student_id: &str,
        semester: Option<&str>,
        output_path: &str,
    ) -> Result<(), String> {
        let mut wtr = csv::Writer::from_path(output_path)
            .map_err(|e| format!("创建CSV文件失败: {}", e))?;

        // 写入表头
        wtr.write_record(&["学号", "姓名", "学期", "科目", "成绩", "等级"])
            .map_err(|e| format!("写入CSV表头失败: {}", e))?;

        let student = system.get_student(student_id)
            .ok_or_else(|| format!("未找到学号为 {} 的学生", student_id))?;

        let grades = if let Some(sem) = semester {
            system.get_student_semester_grades(student_id, sem)
        } else {
            system.get_student_grades(student_id)
        };

        // 写入成绩记录
        for grade in grades {
            wtr.write_record(&[
                &student.id,
                &student.name,
                &grade.semester,
                &grade.subject,
                &grade.score.to_string(),
                grade.get_grade_level(),
            ])
            .map_err(|e| format!("写入CSV记录失败: {}", e))?;
        }

        wtr.flush().map_err(|e| format!("保存CSV文件失败: {}", e))?;
        Ok(())
    }

    // 从CSV文件导入成绩
    pub fn import_grades_from_csv(
        &self,
        system: &mut GradeManagementSystem,
        file_path: &str,
    ) -> Result<(), String> {
        let mut rdr = csv::Reader::from_path(file_path)
            .map_err(|e| format!("打开CSV文件失败: {}", e))?;

        for result in rdr.records() {
            let record = result.map_err(|e| format!("读取CSV记录失败: {}", e))?;
            if record.len() < 6 {
                return Err("CSV记录格式不正确".to_string());
            }

            let student_id = &record[0];
            let subject = &record[3];
            let score = record[4]
                .parse::<f32>()
                .map_err(|_| "成绩格式不正确".to_string())?;
            let semester = &record[2];

            let grade = Grade::new(
                student_id.to_string(),
                subject.to_string(),
                score,
                semester.to_string(),
            );

            system.add_grade(grade)
                .map_err(|e| format!("导入成绩失败: {}", e))?;
        }

        Ok(())
    }
}
