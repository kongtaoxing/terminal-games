use crossterm::event::KeyCode;
use std::{cell::RefCell, time::Instant};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::games::goldminer::item::{Item, ItemType};
use crate::translation::{Language, Translations};
use crate::{
    game_manager::CompileLanguage,
    games::{compiling::Compiling, goldminer::hook::HookState},
    games::game_trait::Game,
};

// 添加游戏状态枚举
#[derive(PartialEq)]
pub enum GameState {
    Welcome,
    Playing,
    Paused,
}

pub struct GoldMiner {
    pub hook_x: f32,
    pub hook_y: f32,
    pub hook_angle: f32,
    pub hook_state: HookState,
    pub score: i32,
    pub last_update: Instant,
    pub items: Vec<Item>,
    pub caught_item: Option<Item>,
    pub window_width: f32,
    pub window_height: f32,
    pub level: i32,
    pub items_collected: i32,
    pub game_state: GameState,  // 添加游戏状态字段
    translations: Translations, // 添加translations字段
    compiling: RefCell<Compiling>,
}

impl GoldMiner {
    /// 创建一个新的游戏实例
    ///
    /// # Returns
    ///
    /// 返回一个初始化好的 Game 结构体
    pub fn new() -> GoldMiner {
        let terminal = crossterm::terminal::size().unwrap();
        let width = terminal.0 as f32;
        let height = terminal.1 as f32;

        let mut game = GoldMiner {
            hook_x: width / 2.0,
            hook_y: 2.0,
            hook_angle: 0.0,
            hook_state: HookState::Idle,
            score: 0,
            last_update: Instant::now(),
            items: Vec::new(),
            caught_item: None,
            window_width: width,
            window_height: height,
            level: 1,
            items_collected: 0,
            game_state: GameState::Welcome, // 初始状态为欢迎界面
            translations: Translations::new(),
            compiling: RefCell::new(Compiling::new()),
        };
        game.generate_items();
        game
    }

    /// 生成游戏中的物品
    ///
    /// 随机生成金子和石头，并将它们放置在游戏区域内
    pub fn generate_items(&mut self) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        self.items.clear();
        self.items_collected = 0;

        // 添加安全检查
        if self.window_width < 20.0 || self.window_height < 10.0 {
            return;
        }

        let margin = 10.0_f32;
        // 确保生成范围至少有1个单位的宽度
        let min_x = f32::min(margin, self.window_width / 4.0);
        let max_x = f32::max(self.window_width - margin, min_x + 1.0);

        // 确保生成范围至少有1个单位的高度
        let min_y = f32::min(15.0, self.window_height / 4.0);
        let max_y = f32::max(self.window_height - 5.0, min_y + 1.0);

        // 修改：扩展到10个关卡的物品生成
        let level = self.level.min(10); // 第10关后难度不再增加

        // 大金子数量：2-6个
        let big_gold_count = 2 + (level - 1) / 2;
        for _ in 0..big_gold_count {
            if min_x < max_x && min_y < max_y {
                self.items.push(Item {
                    x: rng.gen_range(min_x..max_x),
                    y: rng.gen_range(min_y..max_y),
                    item_type: ItemType::Gold,
                    value: 200,
                    size: 2.0,
                    weight: 2.0 + (level as f32 * 0.2),
                });
            }
        }

        // 小金子数量：4-13个
        let small_gold_count = 4 + (level - 1);
        for _ in 0..small_gold_count {
            if min_x < max_x && min_y < max_y {
                self.items.push(Item {
                    x: rng.gen_range(min_x..max_x),
                    y: rng.gen_range(min_y..max_y),
                    item_type: ItemType::Gold,
                    value: 100,
                    size: 1.0,
                    weight: 1.0 + (level as f32 * 0.1),
                });
            }
        }

        // 石头数量：3-12个
        let stone_count = 3 + level - 1;
        for _ in 0..stone_count {
            if min_x < max_x && min_y < max_y {
                let size = rng.gen_range(1.0..2.0);
                self.items.push(Item {
                    x: rng.gen_range(min_x..max_x),
                    y: rng.gen_range(min_y..max_y),
                    item_type: ItemType::Stone,
                    value: -50,
                    size,
                    weight: size * (1.5 + level as f32 * 0.1),
                });
            }
        }
    }

    /// 更新游戏状态
    ///
    /// 处理钩子的移动、物品的捕获以及分数的计算
    pub fn update(&mut self) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().update();
            return;
        }
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.hook_state == HookState::Idle {
            // 修改：扩展到10个关卡的速度变化
            let base_speed = 0.5;
            let level_speed = match self.level {
                1 => 1.0, // 最慢
                2 => 1.1,
                3 => 1.2,
                4 => 1.3,
                5 => 1.4,
                6 => 1.5,
                7 => 1.6,
                8 => 1.7,
                9 => 1.8,
                _ => 2.0, // 第10关及以后最快
            };

            self.hook_angle += delta * base_speed * level_speed;
            if self.hook_angle > std::f32::consts::PI {
                self.hook_angle = -std::f32::consts::PI;
            }
        }

        let swing_range = (self.window_width / 2.0) - 10.0;
        let hook_screen_x = self.hook_x + (self.hook_angle.sin() * swing_range);

        if self.hook_state == HookState::Extending {
            self.hook_y += delta * 20.0;

            if self.caught_item.is_none() {
                let mut item_to_remove = None;
                for item in &self.items {
                    if (hook_screen_x - item.x).abs() < item.size + 1.0
                        && (self.hook_y - item.y).abs() < item.size + 1.0
                    {
                        self.caught_item = Some((*item).clone());
                        item_to_remove = Some((item.x, item.y));
                        self.hook_state = HookState::Retracting;
                        break;
                    }
                }
                if let Some((x, y)) = item_to_remove {
                    self.items.retain(|i| i.x != x || i.y != y);
                }
            }

            if self.hook_y > self.window_height - 5.0 {
                self.hook_state = HookState::Retracting;
            }
        }

        if self.hook_state == HookState::Retracting {
            let base_speed = 15.0;
            let speed = if let Some(item) = &self.caught_item {
                base_speed / item.weight
            } else {
                base_speed
            };

            self.hook_y -= delta * speed;

            if self.hook_y <= 2.0 {
                self.hook_y = 2.0;
                if let Some(item) = self.caught_item.take() {
                    self.score += item.value;
                    self.items_collected += 1;

                    let gold_remaining = self
                        .items
                        .iter()
                        .any(|i| matches!(i.item_type, ItemType::Gold));

                    if !gold_remaining {
                        self.level += 1;
                        self.generate_items();
                    }
                }
                self.hook_state = HookState::Idle;
            }
        }
    }

    /// 处理用户输入
    ///
    /// # Arguments
    ///
    /// * `key` - 用户按下的键
    pub fn handle_input(&mut self, key: KeyCode) {
        match self.game_state {
            GameState::Welcome => {
                if key == KeyCode::Enter {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Playing => match key {
                KeyCode::Char(' ') => {
                    if self.hook_state == HookState::Idle {
                        self.hook_state = HookState::Extending;
                    }
                }
                KeyCode::Char('p') | KeyCode::Esc => {
                    self.game_state = GameState::Paused;
                }
                _ => {}
            },
            GameState::Paused => {
                if key == KeyCode::Char('p') || key == KeyCode::Esc {
                    self.game_state = GameState::Playing;
                }
            }
        }
    }

    /// 渲染游戏界面
    ///
    /// # Arguments
    ///
    /// * `f` - 帧缓冲区
    /// * `area` - 渲染区域
    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        match self.game_state {
            GameState::Welcome => self.render_welcome(f, area),
            GameState::Playing => self.render_game(f, area),
            GameState::Paused => self.render_pause(f, area),
        }
    }

    // 添加处理欢迎界面的渲染函数
    fn render_welcome<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![Span::styled(
                format!(
                    "{} {}!",
                    self.translations.get_text("welcome_to"),
                    self.translations.get_text("goldminer_title")
                ),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("how_to_play")),
            Spans::from(""),
            Spans::from(self.translations.get_text("hook_swing")),
            Spans::from(self.translations.get_text("press_space")),
            Spans::from(self.translations.get_text("catch_gold")),
            Spans::from(self.translations.get_text("big_gold_points")),
            Spans::from(self.translations.get_text("small_gold_points")),
            Spans::from(self.translations.get_text("avoid_stones")),
            Spans::from(self.translations.get_text("collect_all_gold")),
            Spans::from(self.translations.get_text("higher_levels")),
            Spans::from(self.translations.get_text("faster_hook")),
            Spans::from(self.translations.get_text("heavier_items")),
            Spans::from(self.translations.get_text("more_obstacles")),
            Spans::from(""),
            Spans::from(self.translations.get_text("controls_title")),
            Spans::from(self.translations.get_text("space_control")),
            Spans::from(self.translations.get_text("quit_control")),
            Spans::from(""),
            Spans::from(vec![Span::styled(
                self.translations.get_text("press_enter"),
                Style::default().fg(Color::Green),
            )]),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("goldminer_title"),
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    // 将原来的 render 函数改名为 render_game
    fn render_game<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        // 添加最小窗口大小检查
        if area.width < 20 || area.height < 10 {
            let warning = vec![
                Spans::from(vec![Span::styled(
                    "Window too small!",
                    Style::default().fg(Color::Red),
                )]),
                Spans::from(vec![Span::styled(
                    "Please resize",
                    Style::default().fg(Color::Yellow),
                )]),
            ];
            let paragraph = Paragraph::new(warning).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, area);
            return;
        }

        self.window_width = area.width as f32;
        self.window_height = area.height as f32;
        self.hook_x = self.window_width / 2.0;

        let hook_chars = vec!["▼", "▽"];
        let rope_chars = vec!["║", "│"];
        let frame_index = (self.last_update.elapsed().as_millis() / 200) as usize % 2;
        let hook_char = hook_chars[frame_index];
        let rope_char = rope_chars[frame_index];

        let swing_range = (self.window_width / 2.0) - 10.0;
        let hook_screen_x = self.hook_x + (self.hook_angle.sin() * swing_range);

        let mut content = vec![];

        content.push(Spans::from(vec![
            Span::styled(
                format!("{} ", self.translations.get_text("level")),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(self.level.to_string(), Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(
                format!("{} ", self.translations.get_text("score")),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(self.score.to_string(), Style::default().fg(Color::Green)),
        ]));

        for y in 0..area.height {
            let mut line_spans = vec![];
            for x in 0..area.width {
                let mut char_to_draw = ' ';
                let mut style = Style::default();

                // 绘制未抓到的物品
                for item in &self.items {
                    if (x as f32 - item.x).abs() <= item.size
                        && (y as f32 - item.y).abs() <= item.size
                    {
                        match item.item_type {
                            ItemType::Gold => {
                                if item.size > 1.5 {
                                    char_to_draw = '◆';
                                    style = Style::default().fg(Color::Yellow);
                                } else {
                                    char_to_draw = '♦';
                                    style = Style::default().fg(Color::Yellow);
                                }
                            }
                            ItemType::Stone => {
                                if item.size > 1.5 {
                                    char_to_draw = '■';
                                    style = Style::default().fg(Color::Gray);
                                } else {
                                    char_to_draw = '□';
                                    style = Style::default().fg(Color::DarkGray);
                                }
                            }
                            ItemType::Nothing => {}
                        }
                    }
                }

                // 绘制被抓到的物品（保持原来的形状和大小）
                if let Some(caught) = &self.caught_item {
                    let caught_x = hook_screen_x;
                    let caught_y = self.hook_y + caught.size; // 物品位于钩子正下方

                    if (x as f32 - caught_x).abs() <= caught.size
                        && (y as f32 - caught_y).abs() <= caught.size
                    {
                        match caught.item_type {
                            ItemType::Gold => {
                                if caught.size > 1.5 {
                                    char_to_draw = '◆';
                                    style = Style::default().fg(Color::Yellow);
                                } else {
                                    char_to_draw = '♦';
                                    style = Style::default().fg(Color::Yellow);
                                }
                            }
                            ItemType::Stone => {
                                if caught.size > 1.5 {
                                    char_to_draw = '■';
                                    style = Style::default().fg(Color::Gray);
                                } else {
                                    char_to_draw = '□';
                                    style = Style::default().fg(Color::DarkGray);
                                }
                            }
                            ItemType::Nothing => {}
                        }
                    }
                }

                // 绘制钩子和绳子
                if y as f32 == self.hook_y.floor() && x as f32 == hook_screen_x.floor() {
                    line_spans.push(Span::styled(hook_char, Style::default().fg(Color::Red)));
                } else if x as f32 == hook_screen_x.floor() && (y as f32) < self.hook_y {
                    line_spans.push(Span::styled(rope_char, Style::default().fg(Color::Red)));
                } else {
                    line_spans.push(Span::styled(char_to_draw.to_string(), style));
                }
            }
            content.push(Spans::from(line_spans));
        }

        let paragraph = Paragraph::new(content).block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("goldminer_title"),
                Style::default().fg(Color::Yellow),
            )),
        );
        f.render_widget(paragraph, area);
    }

    pub fn render_pause<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().render(f, area);
        }
    }

    pub fn set_language(&mut self, language: Language) {
        self.translations.set_language(language);
    }

    pub fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.compiling.borrow_mut().set_language(lang);
    }
}

impl Game for GoldMiner {
    fn new() -> Self {
        Self::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        self.handle_input(key)
    }

    fn update(&mut self) {
        self.update()
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        self.render(f, area)
    }

    fn set_language(&mut self, language: Language) {
        self.set_language(language)
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.set_compile_language(lang)
    }
}
