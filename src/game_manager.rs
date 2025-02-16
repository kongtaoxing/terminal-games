use crate::games::{goldminer::GoldMiner, tetris::Tetris};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(PartialEq)]
pub enum GameState {
    MainMenu,
    GoldMiner,
    Tetris,
}

pub struct GameManager {
    pub state: GameState,
    goldminer: GoldMiner,
    tetris: Tetris,
    selected_game: usize,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            state: GameState::MainMenu,
            goldminer: GoldMiner::new(),
            tetris: Tetris::new(),
            selected_game: 0,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.state {
            GameState::MainMenu => match key {
                KeyCode::Char('1') => self.state = GameState::GoldMiner,
                KeyCode::Char('2') => self.state = GameState::Tetris,
                KeyCode::Up => {
                    if self.selected_game > 0 {
                        self.selected_game -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected_game < 1 {
                        self.selected_game += 1;
                    }
                }
                KeyCode::Enter => {
                    self.state = match self.selected_game {
                        0 => GameState::GoldMiner,
                        1 => GameState::Tetris,
                        _ => GameState::MainMenu,
                    };
                }
                KeyCode::Char('q') => {}
                _ => {}
            },
            GameState::GoldMiner => {
                let _ = self.goldminer.handle_input(key);
            }
            GameState::Tetris => {
                let _ = self.tetris.handle_input(key);
            }
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::MainMenu => {}
            GameState::GoldMiner => self.goldminer.update(),
            GameState::Tetris => self.tetris.update(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        match self.state {
            GameState::MainMenu => self.render_main_menu(f, area),
            GameState::GoldMiner => self.goldminer.render(f, area),
            GameState::Tetris => self.tetris.render(f, area),
        }
    }

    fn render_main_menu<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let menu_text = vec![
            Spans::from(vec![Span::styled(
                "Terminal Game Collection",
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from("Available Games:"),
            Spans::from(""),
            Spans::from(vec![Span::styled(
                " 1. Gold Miner",
                Style::default().fg(if self.selected_game == 0 {
                    Color::Green
                } else {
                    Color::White
                }),
            )]),
            Spans::from(vec![Span::styled(
                " 2. Tetris",
                Style::default().fg(if self.selected_game == 1 {
                    Color::Green
                } else {
                    Color::White
                }),
            )]),
            Spans::from(""),
            Spans::from(" Controls:"),
            Spans::from(" - Use UP/DOWN arrows to select"),
            Spans::from(" - Press ENTER to start game"),
            Spans::from(" - Q: Quit game"),
            Spans::from(""),
            Spans::from(vec![Span::styled(
                "Use arrows to select and press Enter!",
                Style::default().fg(Color::Yellow),
            )]),
        ];

        let paragraph = Paragraph::new(menu_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Game Menu",
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }
}
