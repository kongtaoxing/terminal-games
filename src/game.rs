use crossterm::event::KeyCode;
use std::time::Instant;
use tui::{
    backend::Backend,
    layout::Rect,
    text::Spans,
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
        game.generate_items();
        game
    }

    pub fn generate_items(&mut self) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        
        self.items.clear();
        
        let margin = 10.0;
        let min_x = (self.window_width / 2.0) - (self.window_width / 2.0) + margin;
        let max_x = (self.window_width / 2.0) + (self.window_width / 2.0) - margin;
        
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

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.hook_state == HookState::Idle {
            self.hook_angle += delta * 2.0;
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
                    if (hook_screen_x - item.x).abs() < item.size && 
                       (self.hook_y - item.y).abs() < item.size {
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
                    
                    if self.items.is_empty() {
                        self.generate_items();
                    }
                }
                self.hook_state = HookState::Idle;
            }
        }
    }

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

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        self.window_width = area.width as f32;
        self.window_height = area.height as f32;
        self.hook_x = self.window_width / 2.0;

        let hook_char = "↓";
        let rope_char = "│";
        let swing_range = (self.window_width / 2.0) - 10.0;
        let hook_screen_x = self.hook_x + (self.hook_angle.sin() * swing_range);
        
        let mut content = vec![];
        content.push(Spans::from(format!("Score: {}", self.score)));
        
        for y in 0..area.height {
            let mut line = String::new();
            for x in 0..area.width {
                let mut char_to_draw = ' ';
                
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