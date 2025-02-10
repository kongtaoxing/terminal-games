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

// 游戏对象结构
pub struct Game {
    hook_x: f32,      // 钩子x坐标
    hook_y: f32,      // 钩子y坐标
    hook_angle: f32,  // 钩子角度
    hook_state: HookState,
    score: i32,       // 分数
    last_update: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            hook_x: 40.0,
            hook_y: 2.0,
            hook_angle: 0.0,
            hook_state: HookState::Idle,
            score: 0,
            last_update: Instant::now(),
        }
    }

    // 更新游戏状态
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // 钩子摆动
        if self.hook_state == HookState::Idle {
            self.hook_angle = (self.hook_angle + delta * 2.0) % std::f32::consts::PI;
        }
        
        // 钩子伸出
        if self.hook_state == HookState::Extending {
            self.hook_y += delta * 20.0;
            if self.hook_y > 20.0 {
                self.hook_state = HookState::Retracting;
            }
        }
        
        // 钩子收回
        if self.hook_state == HookState::Retracting {
            self.hook_y -= delta * 15.0;
            if self.hook_y <= 2.0 {
                self.hook_y = 2.0;
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

    // 渲染游戏画面
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let hook_char = "↓";
        let rope_char = "│";
        
        // 计算钩子位置
        let hook_screen_x = if self.hook_state == HookState::Idle {
            self.hook_x + (self.hook_angle.sin() * 10.0)
        } else {
            self.hook_x + (self.hook_angle.sin() * 10.0)
        };
        
        // 创建游戏画面
        let mut content = vec![];
        
        // 添加分数显示
        content.push(Spans::from(format!("Score: {}", self.score)));
        
        // 添加钩子和绳子
        for y in 0..30 {
            let mut line = String::new();
            for x in 0..80 {
                if (y as f32).floor() == self.hook_y.floor() && (x as f32).floor() == hook_screen_x.floor() {
                    line.push_str(hook_char);
                } else if (x as f32).floor() == hook_screen_x.floor() && ((y as f32) < self.hook_y) {
                    line.push_str(rope_char);
                } else {
                    line.push(' ');
                }
            }
            content.push(Spans::from(line));
        }

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Gold Miner"));
        f.render_widget(paragraph, area);
    }
}
