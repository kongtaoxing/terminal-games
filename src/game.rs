use crossterm::event::KeyCode;
use std::time::Instant;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::hook::HookState;
use crate::item::{Item, ItemType};

pub struct Game {
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
}

impl Game {
    /// 创建一个新的游戏实例
    ///
    /// # Returns
    ///
    /// 返回一个初始化好的 Game 结构体
    pub fn new() -> Game {
        let terminal = crossterm::terminal::size().unwrap();
        let width = terminal.0 as f32;
        let height = terminal.1 as f32;

        let mut game = Game {
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

        let margin = 10.0;
        let min_x = margin;
        let max_x = self.window_width - margin;

        let level = self.level.min(5);

        let big_gold_count = 2 + (level - 1) / 2;
        for _ in 0..big_gold_count {
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height - 5.0),
                item_type: ItemType::Gold,
                value: 200,
                size: 2.0,
                weight: 2.0 + (level as f32 * 0.2),
            });
        }

        let small_gold_count = 4 + (level - 1);
        for _ in 0..small_gold_count {
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height - 5.0),
                item_type: ItemType::Gold,
                value: 100,
                size: 1.0,
                weight: 1.0 + (level as f32 * 0.1),
            });
        }

        let stone_count = 3 + level - 1;
        for _ in 0..stone_count {
            let size = rng.gen_range(1.0..2.0);
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height - 5.0),
                item_type: ItemType::Stone,
                value: -50,
                size,
                weight: size * (1.5 + level as f32 * 0.1),
            });
        }
    }

    /// 更新游戏状态
    ///
    /// 处理钩子的移动、物品的捕获以及分数的计算
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.hook_state == HookState::Idle {
            let base_speed = 1.0;  // 基础速度
            let level_speed = match self.level {
                1 => 1.0,     // 第1关：基础速度
                2 => 1.2,     // 第2关：稍快
                3 => 1.4,     // 第3关：更快
                4 => 1.6,     // 第4关：更快
                _ => 2.0,     // 第5关及以后：最快速度
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
                for item in &self.items {
                    if (hook_screen_x - item.x).abs() < item.size + 1.0 && 
                       (self.hook_y - item.y).abs() < item.size + 1.0 {
                        self.caught_item = Some((*item).clone());
                        self.hook_state = HookState::Retracting;
                        break;
                    }
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
                    self.items.retain(|i| i.x != item.x || i.y != item.y);
                    self.items_collected += 1;

                    if self.items.is_empty() {
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
        match key {
            KeyCode::Char(' ') => {
                if self.hook_state == HookState::Idle {
                    self.hook_state = HookState::Extending;
                }
            }
            _ => {}
        }
    }

    /// 渲染游戏界面
    ///
    /// # Arguments
    ///
    /// * `f` - 帧缓冲区
    /// * `area` - 渲染区域
    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
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
            Span::styled("Level: ", Style::default().fg(Color::Yellow)),
            Span::styled(self.level.to_string(), Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled(self.score.to_string(), Style::default().fg(Color::Green)),
        ]));
        
        for y in 0..area.height {
            let mut line_spans = vec![];
            for x in 0..area.width {
                let mut char_to_draw = ' ';
                let mut style = Style::default();
                
                for item in &self.items {
                    if (x as f32 - item.x).abs() <= item.size && 
                       (y as f32 - item.y).abs() <= item.size {
                        match item.item_type {
                            ItemType::Gold => {
                                if item.size > 1.5 {
                                    char_to_draw = '◆';
                                    style = Style::default().fg(Color::Yellow);
                                } else {
                                    char_to_draw = '♦';
                                    style = Style::default().fg(Color::Yellow);
                                }
                            },
                            ItemType::Stone => {
                                if item.size > 1.5 {
                                    char_to_draw = '■';
                                    style = Style::default().fg(Color::Gray);
                                } else {
                                    char_to_draw = '□';
                                    style = Style::default().fg(Color::DarkGray);
                                }
                            },
                            ItemType::Nothing => {},
                        }
                    }
                }
                
                if y as f32 == self.hook_y.floor() && x as f32 == hook_screen_x.floor() {
                    line_spans.push(Span::styled(
                        hook_char,
                        Style::default().fg(Color::Red)
                    ));
                } else if x as f32 == hook_screen_x.floor() && (y as f32) < self.hook_y {
                    line_spans.push(Span::styled(
                        rope_char,
                        Style::default().fg(Color::Red)
                    ));
                } else {
                    line_spans.push(Span::styled(
                        char_to_draw.to_string(),
                        style
                    ));
                }
            }
            content.push(Spans::from(line_spans));
        }

        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Gold Miner",
                    Style::default().fg(Color::Yellow)
                )));
        f.render_widget(paragraph, area);
    }
}
