use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Tetris {
    // 后续添加具体实现
}

impl Tetris {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_input(&mut self, _key: KeyCode) {
        // 后续添加具体实现
    }

    pub fn update(&mut self) {
        // 后续添加具体实现
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![
                Span::styled("Welcome to Tetris!", 
                    Style::default().fg(Color::Cyan))
            ]),
            Spans::from(""),
            Spans::from("Coming Soon..."),
            Spans::from(""),
            Spans::from("Press Q to return to main menu"),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Tetris",
                    Style::default().fg(Color::Cyan)
                )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }
} 