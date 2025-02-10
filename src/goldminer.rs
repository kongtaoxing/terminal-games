use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

// 游戏状态枚举
#[derive(PartialEq)]
enum HookState {
    Idle,      // 钩子待机
    Extending, // 钩子伸出
    Retracting, // 钩子收回
}

// 添加物品类型枚举
#[derive(Clone, Copy, PartialEq)]
enum ItemType {
    Gold,    // 金子
    Stone,   // 石头
    Nothing, // 空
}

// 添加物品结构
#[derive(Clone)]
struct Item {
    x: f32,
    y: f32,
    item_type: ItemType,
    value: i32,
    size: f32,    // 新增：物品大小
    weight: f32,  // 新增：物品重量
}

// 游戏对象结构
pub struct Game {
    hook_x: f32,      // 钩子x坐标
    hook_y: f32,      // 钩子y坐标
    hook_angle: f32,  // 钩子角度
    hook_state: HookState,
    score: i32,       // 分数
    last_update: Instant,
    items: Vec<Item>,        // 新增：物品列表
    caught_item: Option<Item>, // 新增：当前抓住的物品
    window_width: f32,  // 新增：窗口宽度
    window_height: f32, // 新增：窗口高度
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            hook_x: 40.0,
            hook_y: 2.0,
            hook_angle: 0.0,
            hook_state: HookState::Idle,
            score: 0,
            last_update: Instant::now(),
            items: Vec::new(),
            caught_item: None,
            window_width: 80.0,
            window_height: 30.0,
        };
        game.generate_items(); // 生成初始物品
        game
    }

    // 修改：生成随机物品
    fn generate_items(&mut self) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        
        self.items.clear();
        
        // 修改：物品生成范围与钩子摆动范围保持一致
        let margin = 10.0;
        let min_x = (self.window_width / 2.0) - (self.window_width / 2.0) + margin;
        let max_x = (self.window_width / 2.0) + (self.window_width / 2.0) - margin;
        
        // 生成大金子（更值钱但更重）
        for _ in 0..2 {
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height-5.0),
                item_type: ItemType::Gold,
                value: 200,
                size: 2.0,
                weight: 2.0,
            });
        }
        
        // 生成小金子
        for _ in 0..4 {
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height-5.0),
                item_type: ItemType::Gold,
                value: 100,
                size: 1.0,
                weight: 1.0,
            });
        }
        
        // 生成石头（不同大小）
        for _ in 0..3 {
            let size = rng.gen_range(1.0..2.0);
            self.items.push(Item {
                x: rng.gen_range(min_x..max_x),
                y: rng.gen_range(15.0..self.window_height-5.0),
                item_type: ItemType::Stone,
                value: -50,
                size,
                weight: size * 1.5,
            });
        }
    }

    // 修改：更新游戏状态
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.hook_state == HookState::Idle {
            // 修改：增加摆动范围，使用 -PI 到 PI 的完整范围
            self.hook_angle += delta * 2.0;
            if self.hook_angle > std::f32::consts::PI {
                self.hook_angle = -std::f32::consts::PI;
            }
        }
        
        // 修改：增大摆动范围，使用窗口宽度的一半减去边距
        let swing_range = (self.window_width / 2.0) - 10.0;
        let hook_screen_x = self.hook_x + (self.hook_angle.sin() * swing_range);
        
        // 钩子伸出
        if self.hook_state == HookState::Extending {
            self.hook_y += delta * 20.0;
            
            // 检查碰撞
            if self.caught_item.is_none() {
                for item in &self.items {
                    if (hook_screen_x - item.x).abs() < item.size && 
                       (self.hook_y - item.y).abs() < item.size {
                        self.caught_item = Some((*item).clone());
                        self.hook_state = HookState::Retracting;
                        break;
                    }
                }
            }
            
            // 到达底部则收回
            if self.hook_y > self.window_height - 5.0 {
                self.hook_state = HookState::Retracting;
            }
        }
        
        // 钩子收回（速度受物品重量影响）
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
                // 计算分数并移除抓到的物品
                if let Some(item) = self.caught_item.take() {
                    self.score += item.value;
                    self.items.retain(|i| i.x != item.x || i.y != item.y);
                    
                    // 如果物品都被收集完，重新生成
                    if self.items.is_empty() {
                        self.generate_items();
                    }
                }
                self.hook_state = HookState::Idle;
            }
        }
    }

    // 处理输入
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

    // 修改：渲染游戏画面
    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        // 修改：确保钩子的初始位置在屏幕正中间
        self.window_width = area.width as f32;
        self.window_height = area.height as f32;
        self.hook_x = self.window_width / 2.0;

        // 使用与 update 相同的摆动范围计算
        let swing_range = (self.window_width / 2.0) - 10.0;
        let hook_screen_x = self.hook_x + (self.hook_angle.sin() * swing_range);
        
        let hook_char = "↓";
        let rope_char = "│";
        
        let mut content = vec![];
        content.push(Spans::from(format!("Score: {}", self.score)));
        
        for y in 0..area.height {
            let mut line = String::new();
            for x in 0..area.width {
                let mut char_to_draw = ' ';
                
                // 绘制物品（根据大小使用不同字符）
                for item in &self.items {
                    if (x as f32).floor() == item.x.floor() && 
                       (y as f32).floor() == item.y.floor() {
                        char_to_draw = match item.item_type {
                            ItemType::Gold => if item.size > 1.5 { '@' } else { '$' },
                            ItemType::Stone => if item.size > 1.5 { '#' } else { 'O' },
                            ItemType::Nothing => ' ',
                        };
                    }
                }
                
                // 绘制钩子和绳子
                if y as f32 == self.hook_y.floor() && x as f32 == hook_screen_x.floor() {
                    line.push_str(hook_char);
                } else if x as f32 == hook_screen_x.floor() && (y as f32) < self.hook_y {
                    line.push_str(rope_char);
                } else {
                    line.push(char_to_draw);
                }
            }
            content.push(Spans::from(line));
        }

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Gold Miner"));
        f.render_widget(paragraph, area);
    }
}
