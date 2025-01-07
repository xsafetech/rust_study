mod student;
mod grade;
mod system;
mod io;
mod gui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "成绩管理系统",
        native_options,
        Box::new(|cc| Box::new(gui::GradeManagementApp::new(cc)))
    )
}
