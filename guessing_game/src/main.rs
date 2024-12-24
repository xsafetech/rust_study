use iced::{
    button, text_input, Align, Button, Column, Container, Element, Length, Settings, Text, TextInput, Application, Command, Clipboard, Color, Row,
}; 
use rand::Rng;

/// 程序入口函数
/// 设置窗口大小并启动游戏
pub fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: (500, 400),  // 设置窗口大小为 500x400
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    };
    GuessingGame::run(settings)
}

/// 游戏状态结构体
/// 存储游戏运行时的所有状态数据
struct GuessingGame {
    secret_number: u32,           // 需要猜的目标数字
    guess_input: text_input::State, // 输入框状态
    guess: String,                // 用户输入的猜测值
    attempts: u32,                // 尝试次数
    feedback: String,             // 反馈信息
    guess_button: button::State,  // 提交按钮状态
    reset_button: button::State,  // 重置按钮状态
}

/// 消息枚举
/// 定义用户界面可能触发的所有事件类型
#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),  // 输入框内容改变
    Guess,                 // 提交猜测
    Reset,                 // 重置游戏
}

/// 实现 Iced Application trait
/// 定义应用程序的核心逻辑
impl Application for GuessingGame {
    type Message = Message;
    type Flags = ();
    type Executor = iced::executor::Default;

    /// 初始化游戏状态
    /// 生成随机数并设置初始状态
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let secret_number = rand::thread_rng().gen_range(1..=100);  // 生成1-100的随机数
        (
            GuessingGame {
                secret_number,
                guess_input: text_input::State::new(),
                guess: String::new(),
                attempts: 0,
                feedback: String::from("Enter a number between 1 and 100"),
                guess_button: button::State::new(),
                reset_button: button::State::new(),
            },
            Command::none(),
        )
    }

    /// 返回窗口标题
    fn title(&self) -> String {
        String::from("Guessing Game")
    }

    /// 处理用户界面事件
    /// 根据不同的消息类型更新游戏状态
    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            // 处理输入框内容变化
            Message::InputChanged(value) => {
                self.guess = value;
            }
            // 处理猜测提交
            Message::Guess => {
                if let Ok(guess) = self.guess.trim().parse::<u32>() {
                    // 验证输入范围
                    if guess < 1 || guess > 100 {
                        self.feedback = String::from("Please enter a number between 1 and 100!");
                    } else {
                        self.attempts += 1;
                        // 比较猜测值和目标数字
                        if guess < self.secret_number {
                            self.feedback = String::from("Too low! Try again.");
                        } else if guess > self.secret_number {
                            self.feedback = String::from("Too high! Try again.");
                        } else {
                            self.feedback = format!(
                                "Congratulations! The answer is {}. You tried {} times.", 
                                self.secret_number, 
                                self.attempts
                            );
                        }
                    }
                } else {
                    self.feedback = String::from("Please enter a valid number!");
                }
                self.guess.clear();  // 清空输入框
            }
            // 处理游戏重置
            Message::Reset => {
                self.secret_number = rand::thread_rng().gen_range(1..=100);  // 生成新的随机数
                self.attempts = 0;   // 重置尝试次数
                self.feedback = String::from("Enter a number between 1 and 100");
                self.guess.clear();  // 清空输入框
            }
        }
        Command::none()
    }

    /// 构建用户界面
    /// 定义界面布局和样式
    fn view(&mut self) -> Element<Message> {
        // 创建标题
        let title = Container::new(
            Text::new("Guessing Game")
                .size(40)
        )
        .width(Length::Fill)
        .center_x()
        .padding(20);

        // 创建反馈信息显示
        let feedback = Container::new(
            Text::new(&self.feedback)
                .size(20)
                .color(Color::from_rgb(0.0, 0.5, 0.0))  // 设置绿色
        )
        .width(Length::Fill)
        .center_x()
        .padding(10);

        // 创建输入框
        let guess_input = TextInput::new(
            &mut self.guess_input,
            "Enter a number...",
            &self.guess,
            Message::InputChanged,
        )
        .padding(15)
        .size(20)
        .width(Length::Units(200));

        // 创建提交按钮
        let guess_button = Button::new(
            &mut self.guess_button,
            Text::new("Submit").size(16)
        )
        .padding(10)
        .width(Length::Units(100))
        .on_press(Message::Guess);

        // 创建重置按钮
        let reset_button = Button::new(
            &mut self.reset_button,
            Text::new("Reset").size(16)
        )
        .padding(10)
        .width(Length::Units(100))
        .on_press(Message::Reset);

        // 创建按钮行，包含提交和重置按钮
        let button_row = Row::new()
            .spacing(20)
            .push(guess_button)
            .push(reset_button);

        // 创建主要内容列，包含所有UI元素
        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(title)
            .push(feedback)
            .push(guess_input)
            .push(button_row);

        // 创建根容器，设置整体布局
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}
