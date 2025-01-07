use eframe::egui;
use crate::system::GradeManagementSystem;
use crate::student::Student;
use crate::grade::Grade;
use crate::io::FileIO;
use std::sync::Arc;
use std::sync::Mutex;

pub struct GradeManagementApp {
    system: Arc<Mutex<GradeManagementSystem>>,
    file_io: FileIO,
    // GUI 状态
    selected_student_id: String,
    selected_semester: String,
    selected_subject: String,
    new_student: NewStudentState,
    new_grade: NewGradeState,
    message: Option<String>,
}

#[derive(Default)]
struct NewStudentState {
    id: String,
    name: String,
    class: String,
    major: String,
}

#[derive(Default)]
struct NewGradeState {
    student_id: String,
    subject: String,
    score: String,
    semester: String,
}

impl GradeManagementApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 配置字体
        let mut fonts = egui::FontDefinitions::default();
        
        // 添加中文字体
        fonts.font_data.insert(
            "microsoft_yahei".to_owned(),
            egui::FontData::from_static(include_bytes!("/System/Library/Fonts/PingFang.ttc")),
        );

        // 将中文字体设置为优先字体
        fonts.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "microsoft_yahei".to_owned());

        // 设置等宽字体
        fonts.families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "microsoft_yahei".to_owned());

        // 应用字体配置
        cc.egui_ctx.set_fonts(fonts);

        let system = GradeManagementSystem::new();
        let file_io = FileIO::new("grades.json".to_string());

        // 尝试加载保存的数据
        if let Ok(loaded_system) = file_io.load_from_file() {
            GradeManagementApp {
                system: Arc::new(Mutex::new(loaded_system)),
                file_io,
                selected_student_id: String::new(),
                selected_semester: String::new(),
                selected_subject: String::new(),
                new_student: NewStudentState::default(),
                new_grade: NewGradeState::default(),
                message: None,
            }
        } else {
            GradeManagementApp {
                system: Arc::new(Mutex::new(system)),
                file_io,
                selected_student_id: String::new(),
                selected_semester: String::new(),
                selected_subject: String::new(),
                new_student: NewStudentState::default(),
                new_grade: NewGradeState::default(),
                message: None,
            }
        }
    }

    fn show_message(&mut self, message: String) {
        self.message = Some(message);
    }

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("保存数据").clicked() {
                    let result = {
                        let system = self.system.lock().unwrap();
                        self.file_io.save_to_file(&system)
                    };
                    match result {
                        Ok(()) => self.show_message("数据保存成功".to_string()),
                        Err(_) => self.show_message("数据保存失败".to_string()),
                    }
                }
                if ui.button("导出成绩单").clicked() {
                    if self.selected_student_id.is_empty() {
                        self.show_message("请先选择一个学生".to_string());
                        return;
                    }

                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv"])
                        .save_file() {
                            let semester_option = if !self.selected_semester.is_empty() {
                                Some(self.selected_semester.as_str())
                            } else {
                                None
                            };

                            let result = {
                                let system = self.system.lock().unwrap();
                                self.file_io.export_transcript_to_csv(
                                    &system,
                                    &self.selected_student_id,
                                    semester_option,
                                    path.to_str().unwrap(),
                                )
                            };
                            match result {
                                Ok(()) => self.show_message("成绩单导出成功".to_string()),
                                Err(e) => self.show_message(format!("导出失败: {}", e)),
                            }
                        }
                }
            });
        });
    }

    fn render_student_management(&mut self, ui: &mut egui::Ui) {
        ui.heading("学生管理");
        
        // 添加新学生表单
        ui.group(|ui| {
            ui.label("添加新学生");
            ui.horizontal(|ui| {
                ui.label("学号:");
                ui.text_edit_singleline(&mut self.new_student.id);
            });
            ui.horizontal(|ui| {
                ui.label("姓名:");
                ui.text_edit_singleline(&mut self.new_student.name);
            });
            ui.horizontal(|ui| {
                ui.label("班级:");
                ui.text_edit_singleline(&mut self.new_student.class);
            });
            ui.horizontal(|ui| {
                ui.label("专业:");
                ui.text_edit_singleline(&mut self.new_student.major);
            });

            if ui.button("添加").clicked() {
                let student = Student::new(
                    self.new_student.id.clone(),
                    self.new_student.name.clone(),
                    self.new_student.class.clone(),
                    self.new_student.major.clone(),
                );
                
                let result = {
                    let mut system = self.system.lock().unwrap();
                    system.add_student(student)
                };
                match result {
                    Ok(()) => {
                        self.show_message("学生添加成功".to_string());
                        self.new_student = NewStudentState::default();
                    }
                    Err(e) => self.show_message(format!("添加失败: {}", e)),
                }
            }
        });

        // 显示学生列表
        ui.group(|ui| {
            ui.label("学生列表");
            let students = {
                let system = self.system.lock().unwrap();
                system.get_all_students().into_iter().map(|s| (s.id.clone(), s.name.clone())).collect::<Vec<_>>()
            };
            for (id, name) in students {
                ui.horizontal(|ui| {
                    if ui.selectable_label(
                        self.selected_student_id == id,
                        format!("{} - {}", id, name)
                    ).clicked() {
                        self.selected_student_id = id;
                    }
                });
            }
        });
    }

    fn render_grade_management(&mut self, ui: &mut egui::Ui) {
        ui.heading("成绩管理");

        // 添加新成绩表单
        ui.group(|ui| {
            ui.label("添加新成绩");
            ui.horizontal(|ui| {
                ui.label("学号:");
                ui.text_edit_singleline(&mut self.new_grade.student_id);
            });
            ui.horizontal(|ui| {
                ui.label("科目:");
                ui.text_edit_singleline(&mut self.new_grade.subject);
            });
            ui.horizontal(|ui| {
                ui.label("成绩:");
                ui.text_edit_singleline(&mut self.new_grade.score);
            });
            ui.horizontal(|ui| {
                ui.label("学期:");
                ui.text_edit_singleline(&mut self.new_grade.semester);
            });

            if ui.button("添加").clicked() {
                if let Ok(score) = self.new_grade.score.parse::<f32>() {
                    let grade = Grade::new(
                        self.new_grade.student_id.clone(),
                        self.new_grade.subject.clone(),
                        score,
                        self.new_grade.semester.clone(),
                    );

                    let result = {
                        let mut system = self.system.lock().unwrap();
                        system.add_grade(grade)
                    };
                    match result {
                        Ok(()) => {
                            self.show_message("成绩添加成功".to_string());
                            self.new_grade = NewGradeState::default();
                        }
                        Err(e) => self.show_message(format!("添加失败: {}", e)),
                    }
                } else {
                    self.show_message("成绩格式不正确".to_string());
                }
            }
        });

        // 显示成绩列表
        if !self.selected_student_id.is_empty() {
            ui.group(|ui| {
                ui.label(format!("学号 {} 的成绩列表", self.selected_student_id));
                let grades = {
                    let system = self.system.lock().unwrap();
                    system.get_student_grades(&self.selected_student_id)
                        .iter()
                        .map(|g| (g.semester.clone(), g.subject.clone(), g.score, g.get_grade_level().to_string()))
                        .collect::<Vec<_>>()
                };
                
                for (semester, subject, score, level) in grades {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "学期: {}, 科目: {}, 成绩: {}, 等级: {}",
                            semester, subject, score, level
                        ));
                    });
                }
            });
        }
    }

    fn render_statistics(&mut self, ui: &mut egui::Ui) {
        ui.heading("统计信息");
        
        ui.horizontal(|ui| {
            ui.label("学期:");
            ui.text_edit_singleline(&mut self.selected_semester);
            ui.label("科目:");
            ui.text_edit_singleline(&mut self.selected_subject);
        });

        if !self.selected_semester.is_empty() && !self.selected_subject.is_empty() {
            let stats_map = {
                let system = self.system.lock().unwrap();
                system.get_subject_statistics(&self.selected_subject, &self.selected_semester)
            };
            
            ui.group(|ui| {
                ui.label(format!("{} 学期 {} 科目成绩分布", self.selected_semester, self.selected_subject));
                // 按固定顺序显示等级
                for grade in ["A", "B", "C", "D", "F"] {
                    let count = stats_map.get(grade).copied().unwrap_or(0);
                    ui.label(format!("{} 等级: {} 人", grade, count));
                }
            });
        }
    }
}

impl eframe::App for GradeManagementApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_menu_bar(ui);

            // 显示消息
            if let Some(msg) = &self.message {
                ui.label(msg);
            }

            ui.add_space(10.0);

            // 主要内容区域
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_student_management(ui);
                ui.add_space(10.0);
                self.render_grade_management(ui);
                ui.add_space(10.0);
                self.render_statistics(ui);
            });
        });
    }
}
