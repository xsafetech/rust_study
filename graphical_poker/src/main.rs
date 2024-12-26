// 导入必要的外部依赖
use eframe::egui;  // egui GUI框架
use egui::{TextureHandle, TextureOptions};  // GUI纹理相关组件
use rand::Rng;  // 随机数生成器
use std::collections::HashMap;  // 哈希表数据结构

/// 扑克牌游戏的主要结构体
/// 存储游戏的所有状态数据
struct PokerApp {
    card_textures: HashMap<String, TextureHandle>,  // 存储卡牌图片纹理的哈希表
    deck: Vec<String>,                             // 牌堆中剩余的卡牌
    player_hand: Vec<String>,                      // 玩家手中的卡牌
    back_of_card: Option<String>,                  // 卡牌背面的纹理名称
}

/// 为 PokerApp 实现默认值
impl Default for PokerApp {
    fn default() -> Self {
        PokerApp {
            card_textures: HashMap::new(),
            deck: Vec::new(),
            player_hand: Vec::new(),
            back_of_card: None,
        }
    }
}

/// PokerApp 的具体实现
impl PokerApp {
    /// 创建新的游戏实例
    /// 加载卡牌纹理并初始化牌堆
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = PokerApp::default();
        app.load_card_textures(&cc.egui_ctx);  // 加载所有卡牌图片
        app.reset_deck();                      // 初始化牌堆
        app
    }

    /// 加载所有卡牌的图片纹理
    /// 包括52张扑克牌和1张卡牌背面
    fn load_card_textures(&mut self, ctx: &egui::Context) {
        // 定义所有卡牌文件名
        // H=红桃(Hearts), S=黑桃(Spades), C=梅花(Clubs), D=方块(Diamonds)
        let card_files = [
            "AH.png", "2H.png", "3H.png", "4H.png", "5H.png", "6H.png", "7H.png", "8H.png", "9H.png", "10H.png", "JH.png", "QH.png", "KH.png",
            "AS.png", "2S.png", "3S.png", "4S.png", "5S.png", "6S.png", "7S.png", "8S.png", "9S.png", "10S.png", "JS.png", "QS.png", "KS.png",
            "AC.png", "2C.png", "3C.png", "4C.png", "5C.png", "6C.png", "7C.png", "8C.png", "9C.png", "10C.png", "JC.png", "QC.png", "KC.png",
            "AD.png", "2D.png", "3D.png", "4D.png", "5D.png", "6D.png", "7D.png", "8D.png", "9D.png", "10D.png", "JD.png", "QD.png", "KD.png",
            "BACK.png",  // 卡牌背面
        ];

        // 加载每张卡牌的图片并创建纹理
        for file in card_files {
            let image = load_image(&format!("assets/cards/{}", file)).expect("Failed to load image");
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            let texture_handle = ctx.load_texture(file, color_image, TextureOptions::default());
            self.card_textures.insert(file.replace(".png", ""), texture_handle);
            // 特别处理卡牌背面的纹理
            if file == "BACK.png"{
                self.back_of_card = Some(file.replace(".png", ""));
            }
        }
    }

    /// 重置牌堆
    /// 将所有卡牌（除了背面）放回牌堆
    fn reset_deck(&mut self) {
        self.deck = self.card_textures.keys()
            .filter(|s| s != &"BACK")
            .cloned()
            .collect();
    }
}

/// 实现 eframe::App trait
/// 定义游戏的主要逻辑和界面渲染
impl eframe::App for PokerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Poker");  // 显示标题

            // 检查牌堆是否为空，显示相应信息
            if self.deck.is_empty() {
                ui.label("Deck is Empty. Reset Deck to Deal More");
            } else if ui.button("Deal Card").clicked() {
                // 随机发一张牌给玩家
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..self.deck.len());
                let card = self.deck.remove(index);
                self.player_hand.push(card);
            }

            // 重置牌堆按钮
            if ui.button("Reset Deck").clicked() {
                self.player_hand.clear();  // 清空玩家手牌
                self.reset_deck();         // 重置牌堆
            }

            // 显示卡牌背面
            if let Some(texture) = self.back_of_card.as_ref()
                .and_then(|name| self.card_textures.get(name)) 
            {
                ui.add(egui::Image::new(texture)
                    .fit_to_exact_size(egui::Vec2::new(100.0, 150.0)));
            }

            // 显示牌堆剩余卡牌数量
            ui.label(format!("Cards Remaining: {}", self.deck.len()));

            // 水平排列显示玩家手牌
            ui.horizontal(|ui| {
                for card_name in &self.player_hand {
                    if let Some(texture) = self.card_textures.get(card_name) {
                        ui.add(egui::Image::new(texture)
                            .fit_to_exact_size(egui::Vec2::new(100.0, 150.0)));
                    }
                }
            });
        });
    }
}

/// 加载图片文件
/// 返回动态图片对象或错误
fn load_image(path: &str) -> Result<image::DynamicImage, image::ImageError> {
    image::open(path)
}

/// 程序入口函数
/// 初始化并运行游戏
fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple Poker",                                    // 窗口标题
        native_options,                                    // 默认窗口选项
        Box::new(|cc| Box::new(PokerApp::new(cc))),      // 创建游戏实例
    )
}