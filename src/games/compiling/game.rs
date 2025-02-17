use std::collections::VecDeque;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Compiling {
    messages: VecDeque<String>,
    tick_count: u32,
    display_messages: Vec<String>,
}

impl Compiling {
    pub fn new() -> Self {
        let mut messages = VecDeque::new();
        // 预设编译消息
        let compile_messages = vec![
            "Compiling libc v0.2.169",
            "Compiling proc-macro2 v1.0.93",
            "Compiling unicode-ident v1.0.16",
            "Compiling autocfg v1.4.0",
            "Compiling parking_lot_core v0.8.6",
            "Compiling signal-hook v0.3.17",
            "Compiling lock_api v0.4.12",
            "Compiling signal-hook-registry v1.4.2",
            "Compiling getrandom v0.2.15",
            "Compiling mio v0.7.14",
            "Compiling rand_core v0.6.4",
            "Compiling signal-hook-mio v0.2.4",
            "Compiling parking_lot v0.11.2",
            "Compiling quote v1.0.38",
            "Compiling syn v2.0.98",
            "Compiling crossterm v0.22.1",
            "Compiling tui v0.17.0",
            "Compiling zerocopy-derive v0.7.35",
            "Compiling zerocopy v0.7.35",
            "Compiling ppv-lite86 v0.2.20",
            "Compiling rand_chacha v0.3.1",
            "Compiling rand v0.8.5",
        ];

        messages.extend(compile_messages.iter().map(|&s| s.to_string()));
        
        Self {
            messages,
            tick_count: 0,
            display_messages: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.tick_count += 1;
        
        // 每50ms (假设update每10ms调用一次) 更新一次消息
        if self.tick_count % 5 == 0 {
            if let Some(first_msg) = self.messages.pop_front() {
                self.messages.push_back(first_msg);
            }
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        // 根据区域高度确定显示的消息数量
        let visible_lines = (area.height as usize).saturating_sub(2);
        self.display_messages = self.messages
            .iter()
            .take(visible_lines)
            .map(|msg| msg.to_string())
            .collect();

        let spans: Vec<Spans> = self.display_messages
            .iter()
            .map(|msg| {
                let parts: Vec<&str> = msg.split_whitespace().collect();
                let mut spans = vec![];
                
                // "Compiling" 显示为绿色
                spans.push(Span::styled(
                    "Compiling",
                    Style::default().fg(Color::Green),
                ));
                
                // 剩余部分使用默认颜色
                spans.push(Span::raw(" "));
                spans.push(Span::raw(parts[1..].join(" ")));
                
                Spans::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(spans)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Compiling"));

        f.render_widget(paragraph, area);
    }
}
